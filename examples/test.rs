extern crate httplib;
use httplib::*;

fn main() {
    let mut test = Test::new();
    println!("new");
    let x = test.get();

    println!("x: {}", x);
}

