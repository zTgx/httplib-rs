extern crate libc;

use std::ffi::CStr;

pub mod ffi;

//Client
use crate::ffi::{
    make_client_with_host, make_client_with_host_port, make_client_with_host_port_timeout, get_with_path
};
use crate::ffi::{
    make_server, listen_with,
};

//Server
pub struct Server {
    pub s: *mut ffi::Server,
}

impl Server {
    pub fn new() -> Self {
        Server {
            s: unsafe {
                make_server()
            },
        }
    }

    pub fn listen(&mut self, host: String, port: isize, socket_flags: i32) -> bool {
        unsafe {
            listen_with(self.s, host.as_ptr() as *const i8, port as libc::c_int, socket_flags)
        }
    }
}


//Client
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



















