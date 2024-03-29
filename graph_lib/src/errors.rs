use thiserror::Error;

#[derive(Debug, Error)]
pub enum GraphError {
    #[error("Node with the current index already exists")]
    OccupiedError(),
    #[error("Node with the current index does not exist")]
    NodeNotExist(),
    #[error("Same edge already exists")]
    EdgeExist(),
    #[error("Edge between such nodes does not exist")]
    EdgeNotExist(),
    #[error("Not equal indexes")]
    NotEqualIndexes(),
    #[error("Serialization error: {0}")]
    DftError(#[from] std::io::Error),
    #[error("Error while parsing from str")]
    ParseStrError()
}