use std::cell::RefCell;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::rc::{Rc, Weak};
use crate::edge::Edge;

#[derive(Default)]
pub struct NodeBuilder <'a, H: Hash + Eq, NodeData: Display, EdgeData: Display> {
    data: Option<NodeData>,
    outbound_edges: Vec<Rc<RefCell<Edge<'a, H, EdgeData>>>>,
    inbound_edges: Vec<Weak<RefCell<Edge<'a, H, EdgeData>>>>,
}

impl<'a, H: Hash + Eq, NodeData: Display, EdgeData: Display> NodeBuilder<'a, H, NodeData, EdgeData> {

    pub fn new() -> NodeBuilder<'a, H, NodeData, EdgeData> {
        NodeBuilder {
            data: None,
            inbound_edges: vec![],
            outbound_edges: vec![],
        }
    }

    pub fn data(mut self, data: NodeData) -> NodeBuilder<'a, H, NodeData, EdgeData> {
        self.data = Some(data);
        self
    }

    pub fn outbound_edges(mut self, outbound_edges: Vec<Rc<RefCell<Edge<'a, H, EdgeData>>>>) -> NodeBuilder<H, NodeData, EdgeData> {
        self.outbound_edges = outbound_edges;
        self
    }

    pub fn inbound_edges(mut self, inbound_edges: Vec<Weak<RefCell<Edge<'a, H, EdgeData>>>>) -> NodeBuilder<H, NodeData, EdgeData> {
        self.inbound_edges = inbound_edges;
        self
    }

    pub fn build(self) -> Node<'a, H, NodeData, EdgeData> {
        Node {
            data: self.data,
            outbound_edges: self.outbound_edges,
            inbound_edges: self.inbound_edges,
        }
    }
}

#[derive(Debug)]
pub struct Node <'a, H: Hash + Eq, NodeData: Display, EdgeData: Display> {
    pub(crate) data: Option<NodeData>,
    pub(crate) outbound_edges: Vec<Rc<RefCell<Edge<'a, H, EdgeData>>>>,
    pub(crate) inbound_edges: Vec<Weak<RefCell<Edge<'a, H, EdgeData>>>>,
}

impl<'a, H: Hash + Eq, NodeData: Display, EdgeData: Display> fmt::Display for Node<'a, H, NodeData, EdgeData> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.data {
            Some(data) => {write!(f, "{}", data)}
            None => {write!(f, "None")}
        }
    }
}
