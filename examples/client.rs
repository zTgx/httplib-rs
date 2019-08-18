extern crate httplib;
use httplib::*;
use std::ffi::CStr;

fn main() {
    let mut client = Client::with_host_port_timeout("localhost".to_string(), 9001, 3200);
    //let mut client = Client::with_host_port_timeout("baidu.com".to_string(), 443, 3200);
    let x = client.get_with_path(&"/hi".to_string());

        unsafe {
            println!("status: {}", (*x).status);
            println!("version: {}",  CStr::from_ptr( (*x).version ).to_string_lossy().into_owned());
        }
    // let c_str: &CStr = unsafe { CStr::from_ptr((*x).version) };
    // println!("1");
    // let str_slice: &str = c_str.to_str().unwrap();
    // println!("2");
    // let str_buf: String = str_slice.to_owned();
    // println!("body: {:?}", str_buf);

}
