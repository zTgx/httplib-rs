extern crate httplib;
use httplib::*;

#[no_mangle]
extern "C" fn cb(a: i32, b: ffi::RT)  {
    println!("call back after wrapper, v");
}


fn callback(a: i32, b: i32) -> i32 {
    println!("callback: {:?}/{:?}", a, b);

    33
}


// extern "C" fn callback(target: *mut ffi::RustObject, a: i32) {
//     println!("I'm called from C with value {0}", a);
//     unsafe {
//         // 用回调函数接收的值更新RustObject的值：
//         //(*target).a = a;
//     }
// }


fn main() {
    //let mut t = Box::new(Test::new());
    //t.get(1, cb);

    let x = do_thing(callback);
    println!("x: {:?}", x);

    // 创建回调用到的对象：
    // let mut rust_object = Box::new(ffi::RustObject { a: 5 });
    // unsafe {
    //     register_callback(&mut *rust_object, callback);
    // }

}
