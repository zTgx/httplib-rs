extern crate libc;

use std::ffi::CStr;
use libc::c_void;
use libc::c_int;
pub mod ffi;
pub mod traits;

//Test
pub use crate::ffi::{
    make_test, test_get, register_callback, r_set_content, r_set_redirect,
};

//Client
use crate::ffi::{
    make_client_with_host, make_client_with_host_port, make_client_with_host_port_timeout,
};

//Server
use crate::ffi::{
    make_server, listen_with,
};
pub use traits::{Handler};

pub struct Request {
    pub req: *mut ffi::Request,
}

pub struct Response {
    pub res: *mut ffi::Response,
}

pub fn set_redirect(r: *mut ffi::Response, url: String) {
    unsafe {
        ffi::r_set_redirect(r, url.as_ptr() as *const i8);
    }
}

pub fn set_content(r: *mut ffi::Response, s: String, t: String) {
    unsafe {
        ffi::r_set_content(r, s.as_ptr() as *const i8, t.as_ptr() as *const i8);
    }
}

pub fn set_content_n(r: *mut ffi::Response, s: String, n: isize, t: String) {
    unsafe {
        ffi::r_set_content_n(r, s.as_ptr() as *const i8, n as libc::c_int, t.as_ptr() as *const i8);
    }
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

    pub fn listen(&mut self, host: &String, port: i32, socket_flags: i32) -> bool {
        unsafe {
            let mut host = String::from(host.as_str());
            host.push_str("\0");
            let c_str = CStr::from_bytes_with_nul(host.as_bytes()).expect("CStr::from_bytes_with_nul failed");

            //listen_with(self.s, &c_str, port, socket_flags)
            listen_with(self.s, host.as_ptr() as *const i8, port as libc::c_int, socket_flags)
        }
    }

    pub fn get <F>(&mut self, path: String, cb: F)
    where F:
        Fn(*const ffi::Request, *mut ffi::Response) {
        unsafe {
            ffi::getx(self.s, path.as_ptr() as *const i8,  Server::do_thing_wrapper::<F>);
        }
     }

    extern fn do_thing_wrapper<F>(req: *const ffi::Request, res: *mut ffi::Response)
    where F:
        Fn(*const ffi::Request, *mut ffi::Response) {
    }

    pub fn get_raw(&mut self, path: String, cb: extern fn(req: *const ffi::Request, res: *mut ffi::Response)) {
        unsafe {
            let mut path= String::from(path.as_str());
            path.push_str("\0");
            let c_str = CStr::from_bytes_with_nul(path.as_bytes()).expect("CStr::from_bytes_with_nul failed");

            //ffi::getx(self.s, &c_str,  cb);
            ffi::getx(self.s, path.as_ptr() as *const i8, cb);
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

    pub fn get_with_path(&mut self, path: &String) -> *mut ffi::Response {
        let n = path.len();
        let r = unsafe {
            ffi::get_with_path( self.c, path.as_ptr() as *const libc::c_char, n as i32)
        };

        return r;
        // unsafe { CStr::from_ptr( c_chars ).to_str().unwrap() }
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

    pub fn get (&mut self, path: i32, cb: extern fn(i32, ffi::RT)) {
        unsafe {
            ffi::test_get(self.t, path as *const libc::c_int, Box::new( cb ));
        }
    }

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
