use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::rc::{Rc, Weak};
use crate::edge::Edge;

pub struct NodeBuilder <'a, H: Hash + Eq + Display, NodeData: Display + Clone, EdgeData: Display + Clone> {
    index: &'a H,
    data: Option<NodeData>,
    outbound_edges: HashMap<&'a H, Rc<RefCell<Edge<'a, H, EdgeData>>>>,
    inbound_edges: HashMap<&'a H, Weak<RefCell<Edge<'a, H, EdgeData>>>>,
}

impl<'a, H: Hash + Eq + Display, NodeData: Display + Clone, EdgeData: Display + Clone> NodeBuilder<'a, H, NodeData, EdgeData> {

    pub fn new(index: &'a H) -> NodeBuilder<'a, H, NodeData, EdgeData> {
        NodeBuilder {
            index,
            data: None,
            inbound_edges: HashMap::new(),
            outbound_edges: HashMap::new(),
        }
    }

    pub fn data(mut self, data: NodeData) -> NodeBuilder<'a, H, NodeData, EdgeData> {
        self.data = Some(data);
        self
    }

    pub fn outbound_edges(mut self, outbound_edges: HashMap<&'a H, Rc<RefCell<Edge<'a, H, EdgeData>>>>) -> NodeBuilder<H, NodeData, EdgeData> {
        self.outbound_edges = outbound_edges;
        self
    }

    pub fn inbound_edges(mut self, inbound_edges: HashMap<&'a H, Weak<RefCell<Edge<'a, H, EdgeData>>>>) -> NodeBuilder<H, NodeData, EdgeData> {
        self.inbound_edges = inbound_edges;
        self
    }

    pub fn build(self) -> Node<'a, H, NodeData, EdgeData> {
        Node {
            index: self.index,
            data: self.data,
            outbound_edges: self.outbound_edges,
            inbound_edges: self.inbound_edges,
        }
    }
}

#[derive(Debug)]
pub struct Node <'a, H: Hash + Eq + Display, NodeData: Display + Clone, EdgeData: Display + Clone> {
    pub(crate) index: &'a H,
    pub(crate) data: Option<NodeData>,
    pub(crate) outbound_edges: HashMap<&'a H, Rc<RefCell<Edge<'a, H, EdgeData>>>>,
    pub(crate) inbound_edges: HashMap<&'a H, Weak<RefCell<Edge<'a, H, EdgeData>>>>,
}

impl<'a, H: Hash + Eq + Display, NodeData: Display + Clone, EdgeData: Display + Clone> fmt::Display for Node<'a, H, NodeData, EdgeData> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.data {
            Some(data) => { write!(f, "index: {}, data: {}", &self.index, data) }
            None => { write!(f, "index: {}, data: None", &self.index) }
        }
    }
}
