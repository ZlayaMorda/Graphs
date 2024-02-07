use std::io::Write;
use std::fs::{File, OpenOptions};
use crate::errors::GraphError;
use crate::errors::GraphError::SerializerWriteError;

pub trait DftSerializer {
    fn serialize(&self, path: &str) -> Result<(), GraphError> {
        match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path) {
            Ok(mut file) => {
                self.write_nodes(&mut file).and_then(|_| {
                    if let Err(e) = writeln!(file, "#") {
                        return Err(SerializerWriteError(e))
                    }
                    self.write_edges(&mut file)
                })
            }
            Err(e) => { Err(SerializerWriteError(e)) }
        }
    }

    fn write_nodes(&self, file: &mut File) -> Result<(), GraphError>;
    fn write_edges(&self, file: &mut File) -> Result<(), GraphError>;
}