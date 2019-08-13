extern crate libc;

pub mod ffi;

#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub struct Client(ffi::Client);

impl Client {
    pub fn new(host: String, port: isize, time_t: i64) -> Self {
        Client {
            create_client,
        }
    }
}
