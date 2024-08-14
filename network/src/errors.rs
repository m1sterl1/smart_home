use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Connection error")]
    Connection(#[from] std::io::Error),
    #[error("Error serializing/deserializing data")]
    Serialization(#[from] serde_json::Error),
    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, NetworkError>;
