use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::rc::{Rc, Weak};
use std::str::FromStr;
use crate::edge::Edge;
use crate::errors::GraphError;
use crate::errors::GraphError::ParseStrError;

pub struct NodeBuilder <H, NodeData, EdgeData> where
    H: Hash + Eq + Display + FromStr + Clone,
    NodeData: Display + Clone + FromStr,
    EdgeData: Display + Clone + FromStr
{
    index: H,
    data: Option<NodeData>,
    outbound_edges: HashMap<H, Rc<RefCell<Edge<H, EdgeData>>>>,
    inbound_edges: HashMap<H, Weak<RefCell<Edge<H, EdgeData>>>>,
}

impl<H, NodeData, EdgeData> NodeBuilder<H, NodeData, EdgeData> where
    H: Hash + Eq + Display + FromStr + Clone,
    NodeData: Display + Clone + FromStr,
    EdgeData: Display + Clone + FromStr {

    pub fn new(index: H) -> NodeBuilder<H, NodeData, EdgeData> {
        NodeBuilder {
            index,
            data: None,
            inbound_edges: HashMap::new(),
            outbound_edges: HashMap::new(),
        }
    }

    pub fn data(mut self, data: NodeData) -> NodeBuilder<H, NodeData, EdgeData> {
        self.data = Some(data);
        self
    }

    pub fn outbound_edges(mut self, outbound_edges: HashMap<H, Rc<RefCell<Edge<H, EdgeData>>>>) -> NodeBuilder<H, NodeData, EdgeData> {
        self.outbound_edges = outbound_edges;
        self
    }

    pub fn inbound_edges(mut self, inbound_edges: HashMap<H, Weak<RefCell<Edge<H, EdgeData>>>>) -> NodeBuilder<H, NodeData, EdgeData> {
        self.inbound_edges = inbound_edges;
        self
    }

    pub fn build(self) -> Node<H, NodeData, EdgeData> {
        Node {
            index: self.index,
            data: self.data,
            outbound_edges: self.outbound_edges,
            inbound_edges: self.inbound_edges,
        }
    }
}

#[derive(Debug)]
pub struct Node <H, NodeData, EdgeData> where
    H: Hash + Eq + Display + FromStr + Clone,
    NodeData: Display + Clone + FromStr,
    EdgeData: Display + Clone + FromStr
{
    pub(crate) index: H,
    pub(crate) data: Option<NodeData>,
    pub(crate) outbound_edges: HashMap<H, Rc<RefCell<Edge<H, EdgeData>>>>,
    pub(crate) inbound_edges: HashMap<H, Weak<RefCell<Edge<H, EdgeData>>>>,
}

impl<H, NodeData, EdgeData> Display for Node<H, NodeData, EdgeData> where
    H: Hash + Eq + Display + FromStr + Clone,
    NodeData: Display + Clone + FromStr,
    EdgeData: Display + Clone + FromStr {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.data {
            Some(data) => { write!(f, "index: {}, data: {}", &self.index, data) }
            None => { write!(f, "index: {}, data: None", &self.index) }
        }
    }
}

impl<H, NodeData, EdgeData> FromStr for Node<H, NodeData, EdgeData> where
    H: Hash + Eq + Display + FromStr + Clone,
    NodeData: Display + Clone + FromStr,
    EdgeData: Display + Clone + FromStr {

    type Err = GraphError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (index_str, data_str) = s.split_once(char::is_whitespace).ok_or(ParseStrError())?;
        let index = index_str.parse::<H>().map_err(|_| ParseStrError())?;
        let data = data_str.parse::<NodeData>().map_err(|_| ParseStrError())?;
        Ok(NodeBuilder::new(index).data(data).build())
    }
}

impl<H, NodeData, EdgeData> Node<H, NodeData, EdgeData> where
    H: Hash + Eq + Display + FromStr + Clone,
    NodeData: Display + Clone + FromStr,
    EdgeData: Display + Clone + FromStr {

    pub fn to_dft(&self) -> String {
        match &self.data {
            Some(data) => { format!("{} {}", self.index, data) }
            None => { format!("{} None", self.index) }
        }
    }
}
