#[allow(unused_variables)]
use std::mem;

pub fn numbers() {
    let immutable_number: u8 = 123; // u = unsigned, 8 bits, 0 - 255
    println!("immutable_number = {}", immutable_number); // immutable

    // u = unsigned, 0 to 2^N-1
    // i = signed, -2^(N-1) to -2^(N-1)-1
    let mut mutable_number: i8 = 0; // -128 - 127
    println!("mutable_number = {} before", mutable_number);
    mutable_number = 42;
    println!("mutable_number = {} after", mutable_number);

    // variable without any kind of type
    let default_immutable_number = 123456789; // i32 = 32 bits = 32/8 = 4 bits
    println!("default_number = {}, takes up {} bytes", default_immutable_number, mem::size_of_val(&default_immutable_number));

    // variable without any kind of type
    let mut default_mutable_number = 123456789; // i32 = 32 bits = 32/8 = 4 bits
    println!("default_mutable_number = {} before, takes up {} bytes", default_mutable_number, mem::size_of_val(&default_mutable_number));
    default_mutable_number = -1;
    println!("default_mutable_number = {} after, takes up {} bytes", default_mutable_number, mem::size_of_val(&default_mutable_number));

    // u8, u16, u32, u64
    // i8, i16, i32, i64

    // usize, isize
    let number_isize: isize = -123;
    let size_of_number_isize: usize = mem::size_of_val(&number_isize);
    println!("number_isize = {}, takes up {} bytes, {}-bit OS",
             number_isize, size_of_number_isize, size_of_number_isize*8
    );
}