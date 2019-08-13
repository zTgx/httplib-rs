extern crate httplib;
use httplib::*;

fn main() {
    let _client = Client::new("baidu.com".to_string(), 443, 3200);
}
