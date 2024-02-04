use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::rc::{Rc, Weak};
use crate::edge::Edge;
use crate::errors::AppError;
use crate::errors::AppError::{EdgeExist, NodeNotExist, OccupiedError};
use crate::node::Node;

#[derive(Default, Debug)]
pub struct Graph<'a, H: Hash + Eq, NodeData: Display, EdgeData: Display> {
    nodes: HashMap<H, Node<'a, H, NodeData, EdgeData>>
}

impl <'a, H: Hash + Eq, NodeData: Display, EdgeData: Display> Graph<'a, H, NodeData, EdgeData> {

    pub fn add_node(&mut self, key: H, node: Node<'a, H, NodeData, EdgeData>) -> Result<(), AppError> {
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

    pub fn add_edge(&mut self, edge_data: EdgeData, outbound: &'a H, inbound: &'a H) -> Result<(), AppError> {
        let new_edge : Edge<H, EdgeData> = Edge::new(edge_data, outbound, inbound);
        match self.not_contain_edge_and_nodes(&new_edge) {
            Ok(_) => {
                let new_edge : Rc<RefCell<Edge<H, EdgeData>>> = Rc::new(RefCell::new(new_edge));
                let weak_edge : Weak<RefCell<Edge<H, EdgeData>>> = Rc::downgrade(&new_edge);

                let outbound: &mut Node<H, NodeData, EdgeData> = self.nodes.get_mut(outbound)
                    .expect("node does not exist"); // checked if node exists
                outbound.outbound_edges.push(new_edge);

                let inbound: &mut Node<H, NodeData, EdgeData> = self.nodes.get_mut(inbound)
                    .expect("node does not exist");
                inbound.inbound_edges.push(weak_edge);
                Ok(())
            }
            Err(error) => { Err(error) }
        }
    }

    pub fn not_contain_edge_and_nodes(&self, new_edge: &Edge<H, EdgeData>) -> Result<(), AppError> {
        match self.nodes.get(new_edge.node_out) {
            Some(outbound_node) => {
                match self.nodes.get(new_edge.node_in) {
                    Some(_) => {
                        for edge in &outbound_node.outbound_edges {
                            if edge.borrow().eq(new_edge) {
                                return Err(EdgeExist())
                            }
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