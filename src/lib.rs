extern crate libc;

use std::ffi::CStr;
use libc::c_void;
use libc::c_int;
pub mod ffi;
pub mod traits;

//Test
use crate::ffi::{
    make_test, get_test,
};

//Client
use crate::ffi::{
    make_client_with_host, make_client_with_host_port, make_client_with_host_port_timeout, get_with_path
};

//Server
use crate::ffi::{
    make_server, listen_with, get,
};
pub use traits::{Handler};

pub struct Request {
    pub req: *mut ffi::Request,
}

pub struct Response {
    pub res: *mut ffi::Response,
}


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

    pub fn get (&mut self, regex: String, cb: Box<unsafe extern "C" fn(*mut ffi::Request, *mut ffi::Response)>)
    {
        unsafe {
            let mut h: Box<Handler> = cb;
            let x: *mut Handler = &mut *h;
            let _s = get(self.s, regex.as_ptr() as *const i8, x as *mut Handler);
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



















//////Test
pub struct Test {
    pub t: *mut ffi::Test,
}
impl Test {
    pub fn new() -> Self {
        Test {
            t: unsafe { make_test() },
        }
    }

    // pub fn get<F> (&mut self, path: i32, f: F)
    // where F: Fn(i32) {
    //     let user_data = &f as *const _ as *mut c_void;
    //     unsafe {
    //         ffi::get_test(self.t, path as *const libc::c_int, do_thing_wrapper::<F>);
    //     }
    // }
    //
    // // Shim interface function
    // pub extern fn do_thing_wrapper<F>(closure: *mut c_void, a: *const libc::c_int)
    // where F: Fn(i32) {
    //     let opt_closure = closure as *mut Option<F>;
    //     unsafe {
    //         let _res = (*opt_closure).take().unwrap()(a as i32);
    //         // return res as c_int;
    //     }
    // }
}


// Exposed function to the user of the bindings
pub fn do_thing<F>(f: F) where F: Fn(i32, i32) -> i32 {
    let user_data = &f as *const _ as *mut c_void;
    unsafe {
        ffi::do_thing(do_thing_wrapper::<F>, user_data);
    }

    // Shim interface function
    extern fn do_thing_wrapper<F>(closure: *mut c_void, a: c_int, b: c_int) -> c_int
        where F: Fn(i32, i32) -> i32 {
            let opt_closure = closure as *mut Option<F>;
            unsafe {
                let res = (*opt_closure).take().unwrap()(a as i32, b as i32);
                return res as c_int;
            }
        }
}

