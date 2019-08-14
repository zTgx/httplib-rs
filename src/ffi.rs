/// Client
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

///Client
extern "C" {
    //constructor 
    pub fn make_client_with_host(host: *const libc::c_char) -> *mut Client;
    pub fn make_client_with_host_port(host: *const libc::c_char, port: libc::c_int) -> *mut Client;
    pub fn make_client_with_host_port_timeout(host: *const libc::c_char, port: libc::c_int, time_t: libc::time_t) -> *mut Client;

    //method
    pub fn get_with_path(c: *mut Client, path: *const libc::c_char) -> *const libc::c_char;
}













////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//test
#[repr(C)]
pub struct Test {
    pub x: libc::c_int,
}
impl Test {
    pub fn new() -> Self {
        Test {
            x: 55,
        }
    }
}

extern "C" {
    pub fn make_test() -> *mut Test;
    pub fn make_get(i: libc::c_int, j: libc::c_int) -> libc::c_int;
    pub fn make_get_char(t: *mut Test, c: *const libc::c_char) -> *const libc::c_char;
}
