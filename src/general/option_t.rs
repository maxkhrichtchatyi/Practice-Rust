#![allow(unused_variables)]

pub fn option_t() {
    println!("\n>> {}\n", "Option<T>");

    let x: f64 = 3.0;
    let y: f64 = 1.0;

    // Option -> Some(y) | None
    let result =
        if y != 0.0 {
            Some(x / y)
        } else {
            None
        };

    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("Cannot divide by zero!")
    }

    if let Some(z) = result {
        println!("Result is {}", z);
    }
}