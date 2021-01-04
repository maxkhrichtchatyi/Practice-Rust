#[allow(unused_variables)]
use std::mem;

pub fn booleans() {
    println!("\n>> {}\n", "Booleans");

    let boolean: bool = false; // true
    println!("boolean = {}, takes up {} bytes", boolean, mem::size_of_val(&boolean));
}