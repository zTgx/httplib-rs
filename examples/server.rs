extern crate httplib;
use httplib::*;

fn main() {
    let mut server = Server::new();
    let _lis = server.listen("localhost".to_string(), 9001, 0);
}
