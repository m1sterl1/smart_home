use std::ffi::{c_char, c_float, c_void};

use smart_home::devices::{Socket};


// Socket
// pub fn new(id: &str) -> Self 
// pub fn id(&self) -> &str 
// pub fn turn_on(&mut self) -> Result<()> 
// pub fn turn_off(&mut self) -> Result<()>
// pub fn power_consumption(&self) -> f32 
// pub fn state(&self) -> String

// SocketError
// pub enum SocketError {
//     // Different error types
//     #[error("DeviceErrorState")]
//     DeviceErrorState,
//     #[error("Max voltage value 250V excceed")]
//     MaxVoltageExceed,
// }

// 1. Write socket methods -> types with c-style args
// 2. Write socket error -> c_style From
// 3. handle to main structure

#[repr(transparent)]
struct Str(*const c_char);

#[repr(u32)]
enum SocketError{
    NoError = 0,

}

#[repr(transparent)]
struct SocketHandle(*mut c_void);

type New = unsafe extern "C" fn(Str, *mut SocketHandle) -> SocketError;
type Id = unsafe extern "C" fn(SocketHandle) -> Str;
type TurnOn = unsafe extern "C" fn(SocketHandle) -> SocketError;
type TurnOff = unsafe extern "C" fn(SocketHandle) -> SocketError;
type Power = unsafe extern "C" fn(SocketHandle) -> c_float;
type State = unsafe extern "C" fn(SocketHandle) -> Str;

