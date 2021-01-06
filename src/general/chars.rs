#![allow(unused_variables)]
use std::mem;

pub fn chars() {
    println!("\n>> {}\n", "Chars");

    let single_character = 'x';
    println!("{} is char, size = {} bytes", single_character, mem::size_of_val(&single_character));
}