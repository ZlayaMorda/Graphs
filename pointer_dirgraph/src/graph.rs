use std::cell::{Ref, RefCell};
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::io::Write;
use std::fs::File;
use std::hash::Hash;
use std::rc::{Rc, Weak};
use crate::dft::serializer::DftSerializer;
use crate::edge::Edge;
use crate::errors::GraphError;
use crate::errors::GraphError::{EdgeExist, EdgeNotExist, NodeNotExist, NotEqualIndexes, OccupiedError};
use crate::node::{Node};

#[derive(Default, Debug)]
pub struct Graph<'a, H, NodeData, EdgeData> where
    H: Hash + Eq + Display,
    NodeData: Display + Clone,
    EdgeData: Display + Clone
{
    nodes: HashMap<H, Node<'a, H, NodeData, EdgeData>>
}

impl <'a, H, NodeData, EdgeData> Graph<'a, H, NodeData, EdgeData> where
    H: Hash + Eq + Display,
    NodeData: Display + Clone,
    EdgeData: Display + Clone {

    pub fn add_node(&mut self, index: H, node: Node<'a, H, NodeData, EdgeData>) -> Result<(), GraphError> {
        if self.nodes.get(&index).is_some() {
            Err(OccupiedError())
        } else if index.eq(node.index){
            match self.nodes.insert(index, node) {
                None => Ok(()),
                _ => panic!("Changed value, while exists key")
            }
        } else {
            Err(NotEqualIndexes())
        }
    }

    pub fn get_node(&self, key: H) -> Option<&Node<'a, H, NodeData, EdgeData>> {
        self.nodes.get(&key)
    }

    pub fn add_edge(&mut self, new_edge: Edge<'a, H, EdgeData>) -> Result<(), GraphError> {
        match self.contain_nodes_and_not_edge(new_edge.node_out, new_edge.node_in) {
            Ok(_) => {
                let outbound: &H = new_edge.node_out;
                let inbound: &H = new_edge.node_in;
                let new_edge : Rc<RefCell<Edge<H, EdgeData>>> = Rc::new(RefCell::new(new_edge));
                let weak_edge : Weak<RefCell<Edge<H, EdgeData>>> = Rc::downgrade(&new_edge);
                let outbound_node: &mut Node<H, NodeData, EdgeData> = self.nodes.get_mut(outbound)
                    .expect("node does not exist"); // checked if node exists
                outbound_node.outbound_edges.insert(inbound, new_edge);

                let inbound_node: &mut Node<H, NodeData, EdgeData> = self.nodes.get_mut(inbound)
                    .expect("node does not exist");
                inbound_node.inbound_edges.insert(outbound, weak_edge);
                Ok(())
            }
            Err(error) => { Err(error) }
        }
    }

    pub fn remove_edge(&mut self, outbound: &'a H, inbound: &'a H) -> Result<(), GraphError> {
        match self.nodes.get_mut(outbound) {
            Some(node) => {
                match node.outbound_edges.remove(&outbound) {
                    Some(_) => {
                        match self.nodes.get_mut(inbound) {
                            Some(in_node) => {
                                match in_node.inbound_edges.remove(&inbound) {
                                    Some(_) => { Ok(()) }
                                    None => Err(NodeNotExist())
                                }
                            }
                            None => { Err(NodeNotExist())}
                        }
                    }
                    None => { Err(EdgeNotExist()) }
                }
            }
            None => { Err(NodeNotExist()) }
        }
    }

    fn contain_nodes_and_not_edge(&self, outbound: &H, inbound: &H) -> Result<(), GraphError> {
        match self.nodes.get(outbound) {
            Some(outbound_node) => {
                match self.nodes.contains_key(inbound) {
                    true => {
                        if outbound_node.outbound_edges.contains_key(inbound) {
                            return Err(EdgeExist())
                        }
                        Ok(())
                    }
                    false => Err(NodeNotExist())
                }
            },
            None => Err(NodeNotExist())
        }
    }

    pub fn dfs(&self, s: &'a H) {
        let mut visited: HashSet<&H> = HashSet::new();
        self.recurs_dfs(s, &mut visited);
    }

    fn recurs_dfs(&self, s: &'a H, visited: &mut HashSet<&'a H>) {
        if !visited.contains(s) {
            visited.insert(s);
            if let Some(node) = self.nodes.get(s) {
                println!("Node: {}", node);
                print!("Adjacent inbound:");
                node.inbound_edges.iter().for_each(|edge| print!(" {},", edge.0));
                println!();
                print!("Adjacent outbound:");
                node.outbound_edges.iter().for_each(|edge| print!(" {},", edge.0));
                println!();
                node.outbound_edges.iter().for_each(|edge| self.recurs_dfs(edge.0, visited));
            }
        }
    }
    pub fn get_nodes(&self) -> Vec<&Node<'a, H, NodeData, EdgeData>> {
        self.nodes.values().collect()
    }
    pub fn get_edges(&self) -> Vec<Ref<Edge<'a, H, EdgeData>>> {
        self.nodes.values().flat_map(
            |node| node.outbound_edges.values().map(
                |edge| edge.borrow()
            ).collect::<Vec<Ref<Edge<H, EdgeData>>>>()
        ).collect::<Vec<Ref<Edge<H, EdgeData>>>>()
    }
}

impl <'a, H, NodeData, EdgeData> DftSerializer for Graph<'a, H, NodeData, EdgeData> where
    H: Hash + Eq + Display,
    NodeData: Display + Clone,
    EdgeData: Display + Clone {
    fn write_nodes(&self, file: &mut File) -> Result<(), GraphError> {
        for node in self.get_nodes() {
            writeln!(file, "{}", node.to_dft())?
        }
        Ok(())
    }

    fn write_edges(&self, file: &mut File) -> Result<(), GraphError> {
        for edge in self.get_edges() {
            writeln!(file, "{}", edge.to_dft())?
        }
        Ok(())
    }
}
