use std::fs::{File};
use std::io::{BufRead, BufReader};
use crate::errors::GraphError;

enum Mode {
    Nodes,
    Edges
}

pub trait DftDeserializer {

    fn deserialize(&mut self, path: &str) -> Result<(), GraphError> {
        let mut mode = Mode::Nodes;
        let file: File = File::open(path)?;
        for line in BufReader::new(file).lines() {
            let line = line?;
            if &line == "#" {
                mode = Mode::Edges;
                continue
            }
            match mode {
                Mode::Nodes => self.deserialize_nodes(&line)?,
                Mode::Edges => self.deserialize_edges(&line)?
            }
        }
        Ok(())
    }

    fn deserialize_nodes(&mut self, line: &str) -> Result<(), GraphError>;

    fn deserialize_edges(&mut self, line: &str) -> Result<(), GraphError>;
}