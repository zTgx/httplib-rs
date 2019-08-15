extern crate httplib;
use httplib::*;

/*
extern "C" fn cb(v: *const libc::c_int) {
    unsafe {
        //let val_back = &*v;
        println!("We got back the value: {:?}!", v);
    }
}
*/

fn cb(a: i32, b: i32) -> i32 {
    println!("call back after wrapper, v: {}/{}", a, b);

    return 111;
}

fn main() {
    let x = do_thing( cb );
    println!("x: {:?}", x);
}
