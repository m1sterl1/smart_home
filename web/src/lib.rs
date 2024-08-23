use std::error::Error;

mod server;
mod client;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub use client::SmartHomeClient;
pub use server::server_run;