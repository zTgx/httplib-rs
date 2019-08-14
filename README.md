# [httplib-rs](https://github.com/zTgx/httplib-rs.git)  [![Build Status](https://travis-ci.org/zTgx/httplib-rs.svg?branch=master)](https://travis-ci.org/zTgx/httplib-rs) 

A wrapper around [cpp-httplib](https://github.com/yhirose/cpp-httplib.git), a C++ library by yhirose for HTTP/HTTPS. 

# Usage
Add dependencies
```
[dependencies]
httplib = "0.1.0"
```

```rust
extern crate httplib;
use httplib::*;

fn main() {
    let mut client = Client::with_host_port_timeout("localhost".to_string(), 9001, 3200);
    let x = client.get_with_path(&"/".to_string());

    println!("body: \n{}", x);
}
```
