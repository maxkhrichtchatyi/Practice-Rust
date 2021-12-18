#![allow(unused_variables)]

pub fn casting() {
    let number_one: f64 = 1.23456789;
    let number_two: u32 = 300;
    let casted_number_one: u8 = number_two as u8;
    let casted_number_into_string: String = number_two.to_string();

    println!("u32 casted into u8 integer = {}", casted_number_one); // 300 - 44 (with 0 = 45) = 255
    println!("u32 casted into string = {}", casted_number_into_string);
}