pub mod client;
pub mod server;
mod network_device;

pub use client::{Client, UDPClient, TCPClient};
pub use server::{Server, TCPServer, UDPServer, SharedDevice};
pub use network_device::NetworkDevice;