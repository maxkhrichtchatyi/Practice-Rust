#![allow(unused_variables)]

pub fn while_loop() {
    println!("\n>> {}\n", "While loop");

    let mut incremented_value: i8 = 1;

    while incremented_value <= 10 {
        if incremented_value == 4 {
            incremented_value += 1;
            continue;
        }

        println!("Incremented value = {}", incremented_value);
        incremented_value += 1;
    }

    let mut incremented_value_for_loop: i8 = 1;
    loop { // while true
        println!("Incremented value for loop = {}", incremented_value_for_loop);
        incremented_value_for_loop += 1;

        if incremented_value_for_loop == 10 {
            break;
        }
    }
}