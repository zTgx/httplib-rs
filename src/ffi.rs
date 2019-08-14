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













