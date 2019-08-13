extern crate httplib;
use httplib::*;

fn main() {
    let client = Client::new("baidu.com", 443, 3200);
}
