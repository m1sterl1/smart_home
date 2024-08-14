pub mod r#async;
pub mod command;
mod device;
pub mod errors;
pub mod sync;

pub use errors::Result;

pub const BUFLEN: usize = 1024;
