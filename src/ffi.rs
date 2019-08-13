// use std::ffi::CStr;
// use std::os::raw::c_char;
// use core::{mem};

//    println!("name: {:?}", unsafe { CStr::from_ptr(name) } );

#[repr(C)]
pub struct Client {
    pub host: *const libc::c_char,
    pub port: libc::c_int,
    pub time_t: libc::time_t,
}
impl Client {
    /// Create a new (zeroed) public key usable for the FFI interface
    pub fn new(host: *const libc::c_char, port: libc::c_int, time_t: libc::time_t) -> Client {
        Client {
            host: host,
            port: port,
            time_t: time_t,
        }
    }
}

extern "C" {
    pub fn create_client(host: *const libc::c_char, port: libc::c_int, time_t: libc::time_t) -> Client;
}
