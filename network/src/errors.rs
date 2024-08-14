use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Connection error")]
    Connection(#[from] std::io::Error),
    #[error("Error serializing/deserializing data")]
    Serialization(#[from] serde_json::Error),
    #[error("{0}")]
    Other(String),
}
