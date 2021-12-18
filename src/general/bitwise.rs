#![allow(unused_variables)]

pub fn bitwise() {
    let mut value: u8 = 0b1111_0101u8;
    println!("value in decimal is {}", value);
    println!("value in bits is {:08b}", value);

    value = !value;
    println!("value bitwise NOT {:08b}", value);

    value = value & 0b1111_0111;
    println!("value bitwise AND {:08b}", value);
    println!("bit 6 is {}", value & 0b0100_0000);
    println!("bit 1 is {}", value & 0b0000_0010);

    value = value | 0b0100_0000;
    println!("value bitwise OR {:08b}", value);
    println!("bit 6 is {}", value & 0b0100_0000);

    let xor_prev_value: u8 = value;
    let xor_value: u8 = 0b0101_0101;
    value = value ^ xor_value;
    println!("{:08b} XOR {:08b} = {:08b}", xor_prev_value, xor_value, value);

    let mut shift_prev_value: u8 = value;
    let shift_value: u8 = 4;
    value = shift_prev_value << shift_value;
    println!("{:08b} shifted << left by {} = {:08b}", shift_prev_value, shift_value, value);
    shift_prev_value = value;
    value = shift_prev_value >> shift_value;
    println!("{:08b} shifted >> right by {} = {:08b}", shift_prev_value, shift_value, value);
}