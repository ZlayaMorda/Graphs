use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::rc::{Rc, Weak};
use crate::edge::Edge;
use crate::errors::GraphError;
use crate::errors::GraphError::{EdgeExist, EdgeNotExist, NodeNotExist, OccupiedError};
use crate::node::Node;

#[derive(Default, Debug)]
pub struct Graph<'a, H: Hash + Eq + Display, NodeData: Display + Clone, EdgeData: Display + Clone> {
    nodes: HashMap<H, Node<'a, H, NodeData, EdgeData>>
}

impl <'a, H: Hash + Eq + Display, NodeData: Display + Clone, EdgeData: Display + Clone> Graph<'a, H, NodeData, EdgeData> {

    pub fn add_node(&mut self, key: H, node: Node<'a, H, NodeData, EdgeData>) -> Result<(), GraphError> {
        if self.nodes.get(&key).is_some(){
            Err(OccupiedError())
        } else {
            match self.nodes.insert(key, node) {
                None => Ok(()),
                _ => panic!("Changed value, while exists key")
            }
        }
    }

    pub fn get_node(&self, key: H) -> Option<&Node<'a, H, NodeData, EdgeData>> {
        self.nodes.get(&key)
    }

    pub fn add_edge(&mut self, edge_data: EdgeData, outbound: &'a H, inbound: &'a H) -> Result<(), GraphError> {
        let new_edge : Edge<H, EdgeData> = Edge::new(edge_data, outbound, inbound);
        match self.not_contain_edge_and_nodes(new_edge.node_out, new_edge.node_in) {
            Ok(_) => {
                let new_edge : Rc<RefCell<Edge<H, EdgeData>>> = Rc::new(RefCell::new(new_edge));
                let weak_edge : Weak<RefCell<Edge<H, EdgeData>>> = Rc::downgrade(&new_edge);
                let outbound_node: &mut Node<H, NodeData, EdgeData> = self.nodes.get_mut(outbound)
                    .expect("node does not exist"); // checked if node exists
                outbound_node.outbound_edges.insert(outbound, new_edge);

                let inbound_node: &mut Node<H, NodeData, EdgeData> = self.nodes.get_mut(inbound)
                    .expect("node does not exist");
                inbound_node.inbound_edges.insert(inbound, weak_edge);
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

    pub fn not_contain_edge_and_nodes(&self, outbound: &H, inbound: &H) -> Result<(), GraphError> {
        match self.nodes.get(outbound) {
            Some(outbound_node) => {
                match self.nodes.get(inbound) {
                    Some(_) => {
                        if outbound_node.outbound_edges.contains_key(outbound) {
                            return Err(EdgeExist())
                        }
                        Ok(())
                    },
                    None => Err(NodeNotExist())
                }
            },
            None => Err(NodeNotExist())
        }
    }
}