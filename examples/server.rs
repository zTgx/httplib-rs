extern crate httplib;
use httplib::*;

#[no_mangle]
extern "C" fn callback(_req: *const ffi::Request, _rep: *mut ffi::Response) {
    //callback
    println!("IIIIIIIIIIII am back from cpphttplib...");
}

#[no_mangle]
extern "C" fn t(x: i32) {
    println!("callback T: {:?}", x);
}

fn main() {
    let mut server = Box::new( Server::new() );
    //server.get_t(t);
    server.get("/".to_string(), callback);

    let _lis = server.listen(&"localhost".to_string(), 9001, 0);
}
