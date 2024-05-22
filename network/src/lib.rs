use std::error::Error;

mod command;
mod transport;
mod device;

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;
