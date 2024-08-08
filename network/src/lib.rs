use std::error::Error;

pub mod client_async;
pub mod server_async;

pub mod client;
pub mod command;
mod device;
pub mod network_device;
pub mod server;

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;
pub const BUFLEN: usize = 1024;
