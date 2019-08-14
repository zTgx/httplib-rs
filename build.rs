//! # Build script

// Coding conventions
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![warn(missing_docs)]

extern crate cc;

use std::env;
use std::env::consts::OS;

fn main() {
    // Check whether we can use 64-bit compilation
    let _use_64bit_compilation = if env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap() == "64" {
        let check = cc::Build::new().file("depend/check_uint128_t.c")
                                    .cargo_metadata(false)
                                    .try_compile("check_uint128_t")
                                    .is_ok();
        if !check {
            println!("cargo:warning=Compiling in 32-bit mode on a 64-bit architecture due to lack of uint128_t support.");
        }
        check
    } else {
        false
    };

    // Actual build cpp-httplib
    match OS {
        "macos" => {
            cc::Build::new()
                .file("depend/httplib.cc")
                .cpp(true)
                .flag("-std=c++11")
                .cpp_link_stdlib("c++")
                .compile("libhttp.a");
        },

        _ => {
            cc::Build::new()
                .file("depend/test/test.cc")
                .file("depend/httplib.cc")
                .cpp(true)
                .flag("-std=c++11")
                .cpp_link_stdlib("stdc++")
                .compile("libhttp.a");
        }
    }
}
