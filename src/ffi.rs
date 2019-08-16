use libc::c_void;
use libc::c_int;
use std::ffi::CStr;

use crate::traits::{
    ContentProviderResourceReleaser, Handler,
};

///Headers
#[repr(C)]
pub struct Headers {

}

#[repr(C)]
pub struct Params {

}

#[repr(C)]
pub struct MultipartFiles {

}

#[repr(C)]
pub struct Ranges {

}

#[repr(C)]
pub struct Match {

}

///Request
#[repr(C)]
pub struct Request {
    pub version: *const libc::c_char,
    pub method: *const libc::c_char,
    pub target: *const libc::c_char,
    pub path: *const libc::c_char,

    pub headers: *mut Headers,
    pub body: *const libc::c_char,
    pub params: *mut Params,
    pub files: *mut MultipartFiles,
    pub ranges: *mut Ranges,
    pub matches: *mut Match,
}
impl Request {}

#[repr(C)]
pub struct ContentReceiver {

}
#[repr(C)]
pub struct Progress {

}

#[repr(C)]
pub struct ContentProvider {}

///Response
#[repr(C)]
pub struct Response {
    pub version: *const libc::c_char,
    pub status: libc::c_int,
    pub headers: *mut Headers,
    pub body: *const libc::c_char,

    pub content_receiver: *mut ContentReceiver,
    pub progress: *mut Progress,

    pub content_provider_resource_length: libc::c_ulong,
    pub content_provider: *mut ContentProvider,

    pub content_provider_resource_releaser: ContentProviderResourceReleaser,
}



//////////////////////////////////////////////////////////////////////////////////////////////////
///Server
#[repr(C)]
pub struct Server {
    pub dummy: bool,
}
impl Server{
    pub fn new() -> Self {
        Server {
            dummy: false,
        }
    }
}

#[link(name = "http")]
extern "C" {
    //constructor
    pub fn make_server() -> *mut Server;

    //method
    //pub fn listen_with(s: *mut Server, host: &CStr, port: libc::c_int, socket_flags: libc::c_int) -> bool;
    pub fn listen_with(s: *mut Server, host: *const libc::c_char, port: libc::c_int, socket_flags: libc::c_int) -> bool;
    //pub fn getx(s: *mut Server, reg: &CStr, cb: extern fn(*const Request, *mut Response) );
    pub fn getx(s: *mut Server, reg: *const libc::c_char, cb: extern fn(*const Request, *mut Response) );

    pub fn ttt(s: *mut Server, cb: extern fn(x: i32));

    pub fn register_server_callback(s: *mut Server, cb: extern fn(x: i32));
}


/// Client
#[repr(C)]
pub struct Client {
    pub host: *const libc::c_char,
    pub port: libc::c_int,
    pub time_t: libc::time_t,
}
impl Client {
    pub fn new(host: *const libc::c_char, port: libc::c_int, time_t: libc::time_t) -> Client {
        Client {
            host: host,
            port: port,
            time_t: time_t,
        }
    }
}

///Client
extern "C" {
    //constructor
    pub fn make_client_with_host(host: *const libc::c_char) -> *mut Client;
    pub fn make_client_with_host_port(host: *const libc::c_char, port: libc::c_int) -> *mut Client;
    pub fn make_client_with_host_port_timeout(host: *const libc::c_char, port: libc::c_int, time_t: libc::time_t) -> *mut Client;

    //method
    pub fn get_with_path(c: *mut Client, path: *const libc::c_char) -> *const libc::c_char;
}









//////////////
#[repr(C)]
pub struct RustObject {
    pub a: i32,
        // 其他成员……
        }


#[repr(C)]
pub struct RT {
    pub x: libc::c_int,
}

#[repr(C)]
pub struct Test {
    pub dummy: bool,
}
impl Test {
    pub fn new() -> Self {
        Test {
            dummy: false,
        }
    }
}

#[link(name = "http")]
extern {
    pub fn register_callback(target: *mut RustObject, cb: extern fn(*mut RustObject, i32)) -> i32;
}

//Test
extern "C" {
    pub fn make_test() -> *mut Test;
    pub fn test_get(t: *mut Test, path: *const libc::c_int, cb: Box<extern fn(i32, RT)>);
}

extern "C" {
    // Declare the prototype for the external function
    //pub fn do_thing(cb: extern fn (*const libc::c_int));

    // Declare the prototype for the external function
    pub fn do_thing(cb: extern fn (*mut c_void, c_int, c_int) -> c_int, user_data: *mut c_void);

}
