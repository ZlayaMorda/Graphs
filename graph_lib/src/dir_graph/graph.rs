use std::cell::{Ref, RefCell};
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::io::Write;
use std::fs::File;
use std::hash::Hash;
use std::rc::{Rc, Weak};
use std::str::FromStr;
use crate::dft::deserializer::DftDeserializer;
use crate::dft::serializer::DftSerializer;
use crate::dir_graph::edge::Edge;
use crate::errors::GraphError;
use crate::errors::GraphError::{EdgeExist, EdgeNotExist, NodeNotExist, NotEqualIndexes, OccupiedError};
use crate::dir_graph::node::{Node};

/// Directed graph, nodes collected in HashMap
///
/// # Examples
///```
/// use graph_lib::dir_graph::graph::Graph;
/// use graph_lib::dir_graph::node::{Node, NodeBuilder};
/// use graph_lib::dir_graph::edge::Edge;
///
/// let mut graph: Graph<u32, u32, u32> = Graph::default();
/// graph.add_node(1, NodeBuilder::new(1).data(1).build()).expect("Error while add node");
/// graph.add_node(2, NodeBuilder::new(2).data(2).build()).expect("Error while add node");
/// graph.add_edge(Edge::new(1, 1_u32, 2_u32)).expect("Error while add edge");
///
/// assert_eq!(graph.get_node(&1).expect("Error while get node").get_index(), &1);
///
/// let edge = graph.get_node(&1).expect("Error while get node").get_outbound()
/// .get(&2).expect("Error while get edge");
/// assert_eq!(edge.borrow().get_node_out(), &1);
///
/// graph.remove_node(&1).expect("Error remove node");
/// assert!(graph.get_node(&1).is_none());
/// assert!(graph.get_node(&2).expect("Error get node").get_inbound().get(&1).is_none())
/// ```
#[derive(Default, Debug)]
pub struct Graph<H, NodeData, EdgeData> where
    H: Hash + Eq + Display + FromStr + Clone,
    NodeData: Display + Clone + FromStr,
    EdgeData: Display + Clone + FromStr
{
    nodes: HashMap<H, Node<H, NodeData, EdgeData>>
}

impl <'a, H, NodeData, EdgeData> Graph<H, NodeData, EdgeData> where
    H: Hash + Eq + Display + FromStr + Clone,
    NodeData: Display + Clone + FromStr,
    EdgeData: Display + Clone + FromStr {

    pub fn add_node(&mut self, index: H, node: Node<H, NodeData, EdgeData>) -> Result<(), GraphError> {
        if self.nodes.get(&index).is_some() {
            Err(OccupiedError())
        } else if index.eq(&node.index){
            match self.nodes.insert(index, node) {
                None => Ok(()),
                _ => panic!("Changed value, while exists key")
            }
        } else {
            Err(NotEqualIndexes())
        }
    }

    pub fn remove_node(&mut self, index: &H) -> Result<(), GraphError> {
        let node = self.nodes.get(index).ok_or(NodeNotExist())?;
        let outbound_nodes = node.outbound_edges.to_owned();
        let inbound_nodes = node.inbound_edges.to_owned();

        for outbound in outbound_nodes.iter() {
            if let Some(node) = self.nodes.get_mut(outbound.0) {
                node.inbound_edges.remove(index);
            }
        }
        for inbound in inbound_nodes.iter() {
            if let Some(node) = self.nodes.get_mut(inbound.0) {
                node.outbound_edges.remove(index);
            }
        }
        if self.nodes.remove(index).is_none() {
            Err(NodeNotExist())
        } else { Ok(()) }
    }

    pub fn get_node(&self, key: &H) -> Option<&Node<H, NodeData, EdgeData>> {
        self.nodes.get(key)
    }

    pub fn add_edge(&mut self, new_edge: Edge<H, EdgeData>) -> Result<(), GraphError> {
        match self.contain_nodes_and_not_edge(&new_edge.node_out, &new_edge.node_in) {
            Ok(_) => {
                let outbound: H = new_edge.node_out.clone();
                let inbound: H = new_edge.node_in.clone();
                let new_edge : Rc<RefCell<Edge<H, EdgeData>>> = Rc::new(RefCell::new(new_edge));
                let weak_edge : Weak<RefCell<Edge<H, EdgeData>>> = Rc::downgrade(&new_edge);
                let outbound_node: &mut Node<H, NodeData, EdgeData> = self.nodes.get_mut(&outbound)
                    .expect("node does not exist"); // checked if node exists
                outbound_node.outbound_edges.insert(inbound.clone(), new_edge);

                let inbound_node: &mut Node<H, NodeData, EdgeData> = self.nodes.get_mut(&inbound)
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
                match node.outbound_edges.remove(outbound) {
                    Some(_) => {
                        match self.nodes.get_mut(inbound) {
                            Some(in_node) => {
                                match in_node.inbound_edges.remove(inbound) {
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

    pub fn dfs(&self, s: &H) {
        let mut visited: HashSet<&H> = HashSet::new();
        self.recurs_dfs(s, &mut visited);
    }

    fn recurs_dfs(&'a self, s: &'a H, visited: &mut HashSet<&'a H>) {
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
    pub fn get_nodes(&self) -> Vec<&Node<H, NodeData, EdgeData>> {
        self.nodes.values().collect()
    }
    pub fn get_edges(&self) -> Vec<Ref<Edge<H, EdgeData>>> {
        self.nodes.values().flat_map(
            |node| node.outbound_edges.values().map(
                |edge| edge.borrow()
            ).collect::<Vec<Ref<Edge<H, EdgeData>>>>()
        ).collect::<Vec<Ref<Edge<H, EdgeData>>>>()
    }
}

impl <H, NodeData, EdgeData> DftSerializer for Graph<H, NodeData, EdgeData> where
    H: Hash + Eq + Display + FromStr + Clone,
    NodeData: Display + Clone + FromStr,
    EdgeData: Display + Clone + FromStr {
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

impl <H, NodeData, EdgeData> DftDeserializer for Graph<H, NodeData, EdgeData> where
    H: Hash + Eq + Display + FromStr + Clone,
    NodeData: Display + Clone + FromStr,
    EdgeData: Display + Clone + FromStr {

    fn deserialize_nodes(&mut self, line: &str) -> Result<(), GraphError> {
        let node = line.parse::<Node<H, NodeData, EdgeData>>()?;
        self.add_node(node.index.clone(), node)
    }

    fn deserialize_edges(&mut self, line: &str) -> Result<(), GraphError> {
        let edge = line.parse::<Edge<H, EdgeData>>()?;
        self.add_edge(edge)
    }
}