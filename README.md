# RUST CODES



### To hex and back
```rust
extern crate rustc_serialize;
use rustc_serialize::hex::{FromHex, ToHex};

fn main () {
    let hello_str = "Hello, World".as_bytes().to_hex();
    println!("{}", hello_str);
    let bytes = hello_str.from_hex().unwrap();
    println!("{:?}", bytes);
    let result_str = String::from_utf8(bytes).unwrap();
    println!("{}", result_str);
}

```