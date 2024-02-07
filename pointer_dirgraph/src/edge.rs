use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::str::FromStr;
use crate::errors::GraphError;
use crate::errors::GraphError::ParseStrError;

#[derive(Debug)]
pub struct Edge<H, EdgeData> where
    H: Hash + Eq + Display + FromStr + Clone,
    EdgeData: Display + Clone + FromStr
{
    pub(crate) data: Option<EdgeData>,
    pub(crate) node_out: H,
    pub(crate) node_in: H,
}

impl <'a, H, EdgeData> Edge<H, EdgeData> where
    H: Hash + Eq + Display + FromStr + Clone,
    EdgeData: Display + Clone + FromStr {

    pub fn new(data: EdgeData, node_out: H, node_in: H) -> Edge<H, EdgeData> {
        Edge {
            data: Some(data),
            node_out,
            node_in,
        }
    }

    pub fn to_dft(&self) -> String {
        match &self.data {
            Some(data) => { format!("{} {} {}", self.node_out, self.node_in, data) }
            None => { format!("{} {} None", self.node_out, self.node_in) }
        }
    }
}

impl<'a, H, EdgeData> FromStr for Edge<H, EdgeData> where
    H: Hash + Eq + Display + FromStr + Clone,
    EdgeData: Display + Clone + FromStr {

    type Err = GraphError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (node_out_str, node_in_data_str) = s.split_once(char::is_whitespace).ok_or(ParseStrError())?;
        let (node_in_str, data_str) = node_in_data_str.split_once(char::is_whitespace).ok_or(ParseStrError())?;
        let node_out = node_out_str.parse::<H>().map_err(|_| ParseStrError())?;
        let node_in = node_in_str.parse::<H>().map_err(|_| ParseStrError())?;
        let data = data_str.parse::<EdgeData>().map_err(|_| ParseStrError())?;
        Ok(Edge{data: Some(data), node_in, node_out})
    }
}

impl <'a, H, EdgeData> Display for Edge<H, EdgeData> where
    H: Hash + Eq + Display + FromStr + Clone,
    EdgeData: Display + Clone + FromStr {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.data {
            Some(data) => {write!(f, "{}", data)}
            None => {write!(f, "None")}
        }
    }
}

impl <'a, H, EdgeData> PartialEq for Edge<H, EdgeData> where
    H: Hash + Eq + Display + FromStr + Clone,
    EdgeData: Display + Clone + FromStr {

    fn eq(&self, other: &Self) -> bool {
        self.node_out == other.node_out && self.node_in == other.node_in
    }
}

impl <'a, H, EdgeData> Eq for Edge<H, EdgeData> where
    H: Hash + Eq + Display + FromStr + Clone,
    EdgeData: Display + Clone + FromStr {}
