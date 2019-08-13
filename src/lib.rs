extern crate libc;

pub mod ffi;
use crate::ffi::make_client;

pub struct Client(ffi::Client);

impl Client {
    pub fn new(host: String, port: isize, time_t: i64) -> Self {
        Client (
            unsafe { make_client(host.as_ptr() as *const i8, port as libc::c_int, time_t as libc::time_t) }
        )
    }
}
