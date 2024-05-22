use std::error::Error;

mod command;
mod device;
mod transport;

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;
pub const BUFLEN: usize = 1024;
