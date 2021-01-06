#![allow(unused_variables)]
const MEANING_OF_LIFE: u8 = 42; // no fixed address
static STATIC_MEANING: i32 = 123;

pub fn constants() {
    println!("\n>> {}\n", "Constants");

    println!("MEANING_OF_LIFE = {}", MEANING_OF_LIFE);
    println!("STATIC_MEANING = {}", STATIC_MEANING);
}