extern crate httplib;
use httplib::*;

// #[no_mangle]
// extern "C" fn callback(_req: *const ffi::Request, _rep: *mut ffi::Response) {
//     //callback
//     println!("IIIIIIIIIIII am back from cpphttplib...");
// }

fn callback(_req: *const ffi::Request, rep: *mut ffi::Response) {
    //callback
    println!("IIIIIIIIIIII am back from cpphttplib...");

    set_redirect(rep, "/hi".to_string());
}

fn hi_callback(req: *const ffi::Request, rep: *mut ffi::Response) {
    //callback
    println!("hi_callback...");

    set_content(rep, "Hello, Ruster\n".to_string(), "text/plain".to_string());
}

fn main() {
    let mut server = Box::new( Server::new() );
    // server.get("/".to_string(), callback);
    server.get("/hi".to_string(), hi_callback);

    let _lis = server.listen(&"localhost".to_string(), 9001, 0);
}
