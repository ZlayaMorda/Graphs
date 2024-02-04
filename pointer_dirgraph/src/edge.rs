use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

#[derive(Debug)]
pub struct Edge<'a, H: Hash + Eq, EdgeData: Display> {
    data: Option<EdgeData>,
    node_out: &'a H,
    node_in: &'a H,
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
