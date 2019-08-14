extern crate httplib;
use httplib::*;

unsafe extern "C" fn callback(_req: *mut ffi::Request, _rep: *mut ffi::Response) {
    //callback
    println!("...");
}

fn main() {
    let mut server = Server::new();

    server.get("/".to_string(), Box::new( callback) );

    let _lis = server.listen("localhost".to_string(), 9001, 0);
}
