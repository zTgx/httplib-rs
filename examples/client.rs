extern crate httplib;
use httplib::*;

fn main() {
    let mut client = Client::with_host_port_timeout("localhost".to_string(), 9001, 3200);
    //let mut client = Client::with_host_port_timeout("baidu.com".to_string(), 443, 3200);
    let x = client.get_with_path(&"/".to_string());

    println!("body: \n{}", x);
}
