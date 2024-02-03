use std::cell::RefCell;
use std::hash::Hash;
use std::rc::{Rc, Weak};
use crate::edge::Edge;

pub struct NodeBuilder <H: Hash, D> {
    index: H,
    data: Option<D>,
    outbound_edges: Vec<Rc<RefCell<Edge>>>,
    inbound_edges: Vec<Weak<RefCell<Edge>>>,
}

impl<H: Hash, D> NodeBuilder<H, D> {

    pub fn new(index: H) -> NodeBuilder<H, D> {
        NodeBuilder {
            index,
            data: None,
            inbound_edges: vec![],
            outbound_edges: vec![],
        }
    }

    pub fn data(mut self, data: D) -> NodeBuilder<H, D> {
        self.data = Some(data);
        self
    }

    pub fn outbound_edges(mut self, outbound_edges: Vec<Rc<RefCell<Edge>>>) -> NodeBuilder<H, D> {
        self.outbound_edges = outbound_edges;
        self
    }

    pub fn inbound_edges(mut self, inbound_edges: Vec<Weak<RefCell<Edge>>>) -> NodeBuilder<H, D> {
        self.inbound_edges = inbound_edges;
        self
    }

    pub fn build(self) -> Node<H, D> {
        Node {
            index: self.index,
            data: self.data,
            outbound_edges: self.outbound_edges,
            inbound_edges: self.inbound_edges,
        }
    }
}

pub struct Node <H: Hash, D>{
    index: H,
    data: Option<D>,
    outbound_edges: Vec<Rc<RefCell<Edge>>>,
    inbound_edges: Vec<Weak<RefCell<Edge>>>,
}
