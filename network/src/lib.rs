use std::error::Error;

pub mod r#async;
pub mod sync;

pub mod command;
pub mod network_device;
mod device;

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;
pub const BUFLEN: usize = 1024;
