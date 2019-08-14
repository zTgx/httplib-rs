use libc::c_void;
use libc::c_int;

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
extern "C" {
    //constructor
    pub fn make_server() -> *mut Server;

    //method
    pub fn listen_with(s: *mut Server, host: *const libc::c_char, port: libc::c_int, socket_flags: libc::c_int) -> bool;
    pub fn get(s: *mut Server, reg: *const libc::c_char, handler: *mut Handler) -> *mut Server;
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

#[repr(C)]
pub struct Tmp {
    pub callback: extern "C" fn(i: i32),
}

#[no_mangle]
pub extern "C" fn set_callback(callback: extern "C" fn(i: i32)) -> *mut Tmp {
    let mut tmp = Box::new(Tmp { callback });
    println!("tmp as ptr: {:p}", tmp); // >> here <<

    Box::into_raw(tmp)
}

#[no_mangle]
pub extern "C" fn use_callback(tmp_ptr: *mut Tmp) {
    unsafe {
        ((*tmp_ptr).callback)(1);
        ((*tmp_ptr).callback)(3);
    }
}

//Test
extern "C" {
    pub fn make_test() -> *mut Test;
    pub fn get_test(t: *mut Test, path: *const libc::c_int, cb: extern fn(*const libc::c_int));
}

extern "C" {
  // Declare the prototype for the external function
  pub fn do_thing(cb: extern fn (*mut c_void, c_int, c_int) -> c_int,
              user_data: *mut c_void);
}
