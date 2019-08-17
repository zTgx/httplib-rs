extern crate httplib;
use httplib::*;
use std::ffi::CString;
use std::ffi::CStr;

fn cast_ptr_to_string(x: *const i8) -> String {
    let c_str: &CStr = unsafe { CStr::from_ptr(x) };
    println!("c_str: {:?}", c_str);
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned();

    str_buf
}

fn print_request(req: *const ffi::Request) {

    println!("print_request");
    unsafe {
        // let version = cast_ptr_to_string((*req).version);
        // println!("version: {}", (*req).version);

        // let method = cast_ptr_to_string((*req).method);
        // println!("method: {}", method);
        //
        // let target = cast_ptr_to_string((*req).target);
        // println!("target: {}", target);
        //
        // let path = cast_ptr_to_string((*req).path);
        // println!("path: {}", path);
    }
}

#[no_mangle]
extern "C" fn callback(req: *const ffi::Request, rep: *mut ffi::Response) {
    //callback
    println!("IIIIIIIIIIII am back from cpphttplib...");

    set_redirect(rep, "/hi".to_string());
}

#[no_mangle]
extern "C" fn hi_callback(req: *const ffi::Request, rep: *mut ffi::Response) {
    //callback
    println!("hi_callback...");

    set_content(rep, "Hello, Ruster\n".to_string(), "text/plain".to_string());
}

fn main() {
    let mut server = Box::new( Server::new() );
    server.get_raw("/".to_string(), callback);
    server.get_raw("/hi".to_string(), hi_callback);

    let _lis = server.listen(&"localhost".to_string(), 9001, 0);
}
