extern crate libc;

use std::ffi::CStr;

pub mod ffi;

//Client
use crate::ffi::{
    make_client_with_host, make_client_with_host_port, make_client_with_host_port_timeout, get_with_path
};

pub struct Client {
    pub c: *mut ffi::Client
}

impl Client {
    pub fn with_host(host: String) -> Self {
        Client {
            c: unsafe { 
                make_client_with_host( host.as_ptr() as *const i8)
            },
        }
    }

    pub fn with_host_port(host: String, port: isize) -> Self {
        Client {
            c: unsafe {
                make_client_with_host_port( host.as_ptr() as *const i8, port as libc::c_int)
            },
        }
    }

    pub fn with_host_port_timeout(host: String, port: isize, time_t: i64) -> Self {
        Client {
            c: unsafe { 
                make_client_with_host_port_timeout( host.as_ptr() as *const i8, port as libc::c_int, time_t as libc::time_t ) 
            },
        }
    }

    pub fn get_with_path(&mut self, path: &String) -> &str {
        let c_chars = unsafe{ 
            get_with_path( self.c, path.as_ptr() as *const i8) 
        };

        unsafe { CStr::from_ptr( c_chars ).to_str().unwrap() }
    }
}




















/////////////////////////////////////////////////////////////////////////////////////////////////////////
//Test
use crate::ffi::{make_test, make_get_char};
pub struct Test {
    pub t: *mut ffi::Test
}

impl Test {
    pub fn new() -> Self {
        Test {
            t: unsafe{ make_test() }
        }
    }

    pub fn get(&mut self) -> &str {
        unsafe{ CStr::from_ptr( make_get_char( self.t, "".as_ptr() as *const i8) ).to_str().unwrap() }
    }
}
