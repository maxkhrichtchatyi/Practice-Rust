#![allow(unused_variables)]

pub fn while_loop() {
    let mut incremented_value: i8 = 1;

    while incremented_value <= 100 {
        println!("Incremented value = {}", incremented_value);
        incremented_value += 1;
    }
}