#![allow(unused_variables)]

pub fn operators() {
    println!("\n>> {}\n", "Operators");
    println!(">>>> {}\n", "Arithmetic operators");

    let mut arithmetic_var_a = 2 + 3 * 4; // + - * /
    println!("arithmetic_var_a = {}", arithmetic_var_a);

    arithmetic_var_a = arithmetic_var_a + 1;
    println!("arithmetic_var_a + 1 = {}", arithmetic_var_a);

    arithmetic_var_a -= 2;
    println!("arithmetic_var_a -= 2 = {}", arithmetic_var_a);

    println!("reminder of {} / {} is {}", 13, 5, (13 % 5));

    let cubed: i32 = i32::pow(13, 3);
    println!("{} cubed is {}", 13, cubed);

    let float_number: f64 = 2.5;
    let float_number_cubed: f64 = f64::powi(float_number, 3);
    let float_number_to_pi: f64 = f64::powf(float_number, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", float_number, float_number_cubed, float_number, float_number_to_pi);

    println!("\n>>>> {}\n", "Bitwise operators");

    let bitwise_example = 1 | 2;
    // | OR, && AND, ^ XOR, ! NOR
    // 1 | 2 => 01 OR 10
    // 11 => 3 in decimal
    println!("1|2 = {}", bitwise_example);

    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);

    println!("\n>>>> {}\n", "Logical operators");

    let pi_less_4: bool = std::f64::consts::PI < 4.0;
    println!("PI is less then 4 = {}", pi_less_4);
}