use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

#[derive(Debug)]
pub struct Edge<'a, H: Hash + Eq, EdgeData: Display> {
    pub(crate) data: Option<EdgeData>,
    pub(crate) node_out: &'a H,
    pub(crate) node_in: &'a H,
}

impl <'a, H: Hash + Eq, EdgeData: Display> Edge<'a, H, EdgeData> {

    pub fn new(data: EdgeData, node_out: &'a H, node_in: &'a H) -> Edge<'a, H, EdgeData> {
        Edge {
            data: Some(data),
            node_out,
            node_in,
        }
    }
}

impl <'a, H: Hash + Eq, EdgeData: Display> fmt::Display for Edge<'a, H, EdgeData> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.data {
            Some(data) => {write!(f, "{}", data)}
            None => {write!(f, "None")}
        }
    }
}

impl <'a, H: Hash + Eq, EdgeData: Display> PartialEq for Edge<'a, H, EdgeData> {
    fn eq(&self, other: &Self) -> bool {
        self.node_out == other.node_out && self.node_in == other.node_in
    }
}

impl <'a, H: Hash + Eq, EdgeData: Display> Eq for Edge<'a, H, EdgeData> {}
