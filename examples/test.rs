extern crate httplib;
use httplib::*;

fn main() {
    do_thing(|a, b| {
        println!("callback...{}/{}", a, b);

        return 33;
    });
}
