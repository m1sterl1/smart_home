#![allow(dead_code)]
#![allow(unused_variables)]
mod socket;
mod therm;
mod utils;

pub use socket::{Socket, SocketState, SocketError};
pub use therm::{Thermometer, ThermometerState, ThermometerError};
