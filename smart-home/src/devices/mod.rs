#![allow(dead_code)]
#![allow(unused_variables)]
mod socket;
mod therm;
mod utils;

pub use socket::{Socket, SocketError, SocketState};
pub use therm::{Thermometer, ThermometerError, ThermometerState};
