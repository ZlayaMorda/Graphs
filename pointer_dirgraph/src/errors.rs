use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Node with the current index already exists")]
    OccupiedError(),
    #[error("Node with the current index does not exist")]
    NodeNotExist(),
    #[error("Same edge already exists")]
    EdgeExist(),
}