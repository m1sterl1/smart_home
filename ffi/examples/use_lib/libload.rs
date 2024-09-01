use core::str;
use std::error::Error;
use std::ffi::{c_char, c_float, c_void, CString};
use std::fmt::Debug;

use libloading::{Library, Symbol};
use thiserror::Error;

const LIBRARY: &str = "target/release/libffi.so";

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

// Library structure description

#[repr(transparent)]
struct Str(*const c_char);

#[repr(transparent)]
#[derive(Clone, Copy)]
struct SocketHandle(*mut c_void);

impl SocketHandle {
    fn null() -> Self {
        Self(std::ptr::null_mut())
    }
}

#[repr(u32)]
#[derive(PartialEq, Error, Debug)]
enum SocketError {
    #[error("No error")]
    NoError = 0,
    #[error("Parameter error")]
    Param,
    #[error("Device error state")]
    DeviceErrorState,
    #[error("Max voltage exceed")]
    MaxVoltageExceed,
}

type New = unsafe extern "C" fn(Str, *mut SocketHandle) -> SocketError;
type TurnOn = unsafe extern "C" fn(SocketHandle) -> SocketError;
type TurnOff = unsafe extern "C" fn(SocketHandle) -> SocketError;
type Power = unsafe extern "C" fn(SocketHandle, *mut c_float) -> SocketError;

// Loading wrapper

struct Lib {
    lib: Library,
}

impl Lib {
    unsafe fn new(path: &str) -> Result<Self> {
        let library = Library::new(path)?;

        Ok(Self { lib: library })
    }

    unsafe fn new_socket(&self, id: &str) -> Result<SocketHandle> {
        let id = CString::new(id)?;
        let handle = SocketHandle::null();
        let handle = &handle as *const SocketHandle as *mut SocketHandle;
        let new_socket: Symbol<New> = self.lib.get(b"new")?;
        match new_socket(Str(id.as_ptr()), handle) {
            SocketError::NoError => Ok(*handle),
            e => Err(e.into()),
        }
    }

    unsafe fn turn_on(&self, handle: SocketHandle) -> Result<()> {
        let turn_on: Symbol<TurnOn> = self.lib.get(b"turn_on")?;
        match turn_on(handle) {
            SocketError::NoError => Ok(()),
            _ => Err("Error".into()),
        }
    }

    unsafe fn turn_off(&self, handle: SocketHandle) -> Result<()> {
        let turn_off: Symbol<TurnOff> = self.lib.get(b"turn_off")?;
        match turn_off(handle) {
            SocketError::NoError => Ok(()),
            _ => Err("Error".into()),
        }
    }

    unsafe fn power(&self, handle: SocketHandle) -> Result<f32> {
        let power: Symbol<Power> = self.lib.get(b"power")?;
        let mut p: c_float = 0.0;
        match power(handle, &mut p as *mut c_float) {
            SocketError::NoError => Ok(p),
            _ => Err("Error".into()),
        }
    }
}

pub struct SocketWrapper {
    lib: Lib,
    handle: SocketHandle,
}

impl SocketWrapper {
    pub fn new(id: &str) -> Result<Self> {
        unsafe {
            let lib = Lib::new(LIBRARY)?;
            let handle = lib.new_socket(id)?;
            Ok(Self { lib, handle })
        }
    }

    pub fn turn_on(&self) -> Result<()> {
        unsafe {
            self.lib.turn_on(self.handle)?;
        }
        Ok(())
    }
    pub fn turn_off(&self) -> Result<()> {
        unsafe {
            self.lib.turn_off(self.handle)?;
        }
        Ok(())
    }
    pub fn power(&self) -> Result<f32> {
        unsafe {
            let p = self.lib.power(self.handle)?;
            Ok(p)
        }
    }
}
