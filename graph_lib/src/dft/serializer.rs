use std::io::Write;
use std::fs::{File, OpenOptions};
use crate::errors::GraphError;

pub trait DftSerializer {
    fn serialize(&self, path: &str) -> Result<(), GraphError> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        
        self.write_nodes(&mut file).and_then(|_| {
            writeln!(file, "#")?;
            self.write_edges(&mut file)
        })
    }

    fn write_nodes(&self, file: &mut File) -> Result<(), GraphError>;
    fn write_edges(&self, file: &mut File) -> Result<(), GraphError>;
}