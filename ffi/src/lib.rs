use core::str;
use std::ffi::{c_char, c_float, c_void, CStr, CString};
use std::fmt::Debug;
use smart_home::devices::{Socket, self};


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
// 4. exported functions

#[repr(transparent)]
struct Str(*const c_char);

// Errors

impl From<devices::SocketError> for SocketError{
    fn from(value: devices::SocketError) -> Self {
        match value{
            devices::SocketError::DeviceErrorState => SocketError::DeviceErrorState,
            devices::SocketError::MaxVoltageExceed => SocketError::MaxVoltageExceed,
        }
    }
}

#[repr(u32)]
#[derive(Debug, PartialEq)]
enum SocketError{
    NoError = 0,
    Param,
    DeviceErrorState,
    MaxVoltageExceed,
}

// SocketHandle

#[repr(transparent)]
#[derive(Clone, Copy)]
struct SocketHandle(*mut c_void);

impl SocketHandle{
    fn from_socket(socket: Socket) -> Self{
        // place to heap
        let socket = Box::leak(Box::new(socket));
        // get ptr from Box and convert it to *mut c_void
        let socket = socket as *mut Socket as *mut c_void;
        Self(socket)
    }

    fn as_socket(&self) -> &mut Socket{
        unsafe {&mut *(self.0 as *mut Socket)}
    }

    fn null() -> Self{
        Self(std::ptr::null_mut())
    }

}

// type New = unsafe extern "C" fn(Str, *mut SocketHandle) -> SocketError;
// type Id = unsafe extern "C" fn(SocketHandle, *mut Str) -> SocketError;
// type TurnOn = unsafe extern "C" fn(SocketHandle) -> SocketError;
// type TurnOff = unsafe extern "C" fn(SocketHandle) -> SocketError;
// type Power = unsafe extern "C" fn(SocketHandle, *mut c_float) -> SocketError;
// type State = unsafe extern "C" fn(SocketHandle, *mut Str) -> SocketError;


// // Exported functions
// struct Functions{
//     new: New,
//     id: Id,
//     turn_on: TurnOn,
//     turn_off: TurnOff,
//     power: Power,
//     state: State,
// }

// impl Default for Functions{
//     fn default() -> Self {
//         Self { 
//             new: , 
//             id: (), 
//             turn_on: (), 
//             turn_off: (), 
//             power: (), 
//             state: () 
//         }
//     }
// }

// #[no_mangle]
// pub extern "C" fn functions() -> Functions{

// }


/// Creates new Socket with id, 
/// wraps it with SocketHandle, 
/// write SocketHandle to handle *mut c_void pointer
#[no_mangle]
pub unsafe extern "C" fn new (id: Str, handle: *mut SocketHandle) -> SocketError{
    let id = unsafe{
        match CStr::from_ptr(id.0).to_str(){
            Ok(id) => id,
            Err(_) => {
                return SocketError::Param;
            }
        }
    };
    let socket = Socket::new(id);
    *handle = SocketHandle::from_socket(socket);
    SocketError::NoError

}

/// Turn on socket
/// access to it through handle
#[no_mangle]
pub unsafe extern "C" fn turn_on(handle: SocketHandle) -> SocketError{
    match handle.as_socket().turn_on(){
        Ok(_) => SocketError::NoError,
        Err(e) => e.into() 
    }
}

/// Turn off socket
/// access to it through handle
#[no_mangle]
pub unsafe extern "C" fn turn_off(handle: SocketHandle) -> SocketError{
    match handle.as_socket().turn_off(){
        Ok(_) => SocketError::NoError,
        Err(e) => e.into() 
    }
}

#[no_mangle]
/// Get power consumption from socket
pub unsafe extern "C" fn power(handle: SocketHandle, power: *mut c_float) -> SocketError{
    let p = handle.as_socket().power_consumption();
    *power = p;
    SocketError::NoError
}



#[cfg(test)]
mod tests{

    use super::*;

    fn print_memory<T>(desc: &str, reference: &T, number: isize){
        println!("{desc}");
        let reference = reference as *const _ as *const usize;
        for i in 0..number{
            let r = unsafe {reference.offset(i)};
            println!("{:p} {:016x}", r, unsafe{*r});
        }

    }

    #[test]
    fn test_valid_hangle_written(){
        let id = "mysocketid";
        let id_cstring = CString::new("mysocketid").unwrap();
        let id_str = Str(id_cstring.as_ptr());

        let handle = SocketHandle::null();
        print_memory("SocketHandle null", &handle.0, 1);

        let handle = &handle as *const SocketHandle as *mut SocketHandle;
        print_memory("*mut SocketHandle", &handle, 1);

        assert_eq!(unsafe {new(id_str, handle)}, SocketError::NoError);

        let handle = unsafe{&*handle};
        print_memory("Handle After new", &handle.0, 1);

        let socket = handle.as_socket();
        print_memory("Socket from handle", socket, 4);

        assert_eq!(id, socket.id())

    }

    #[test]
    fn test_power_consumption(){
        let id = "mysocketid";
        let id = CString::new(id).unwrap();
        let handle = SocketHandle::null();
        let handle = &handle as *const SocketHandle as *mut SocketHandle;

        assert_eq!(unsafe {new(Str(id.as_ptr()), handle)}, SocketError::NoError);
        let mut p:c_float = 0.0;
        assert_eq!(unsafe{power(*handle, &mut p as *mut c_float)}, SocketError::NoError);
        assert_eq!(0.0, p);
        assert_eq!(unsafe{turn_on(*handle)}, SocketError::NoError);
        assert_eq!(unsafe{power(*handle, &mut p as *mut c_float)}, SocketError::NoError);
        assert_ne!(0.0, p);

    }
}
