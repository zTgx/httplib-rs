extern crate httplib;
use httplib::*;

// #[no_mangle]
// extern "C" fn callback(_req: *const ffi::Request, _rep: *mut ffi::Response) {
//     //callback
//     println!("IIIIIIIIIIII am back from cpphttplib...");
// }

fn callback(_req: *const ffi::Request, _rep: *mut ffi::Response) {
    //callback
    println!("IIIIIIIIIIII am back from cpphttplib...");
}

fn main() {
    let mut server = Box::new( Server::new() );
    // server.get("/".to_string(), callback);
    server.get("/".to_string(), |req, res| {

    });
    let _lis = server.listen(&"localhost".to_string(), 9001, 0);
}
