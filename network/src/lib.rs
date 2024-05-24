use std::error::Error;

pub mod command;
mod device;
pub mod transport;
pub mod network_device;

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;
pub const BUFLEN: usize = 1024;
