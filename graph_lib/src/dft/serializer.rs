use std::io::Write;
use std::fs::{File, OpenOptions};
use crate::errors::GraphError;

/// Graph may implement DftSerializer for the dft serialization
/// Must realize write_nodes and write_edges functions
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

    /// Write all nodes to the file in the dft format
    fn write_nodes(&self, file: &mut File) -> Result<(), GraphError>;

    /// Write all edges to the file in the dft format
    fn write_edges(&self, file: &mut File) -> Result<(), GraphError>;
}