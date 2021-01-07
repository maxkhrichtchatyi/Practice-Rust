#![allow(unused_variables)]

pub fn if_statement() {
    println!("\n>> {}\n", "If Statement");

    let temp: i32 = 45;

    if temp > 30 {
        println!("Really hot outside!");
    } else if temp < 10 {
        println!("Really cold!");
    } else {
        println!("Temperature is OK!")
    }

    let day: &str = if temp > 20 { "Sunny" } else { "Cloudy" };
    println!("Today is {}", day);

    println!("Is it {}",
             if temp > 20 { "Hot" } else if temp < 10 { "Cold" } else { "Ok" }
    );

    println!("It is {}",
             if temp > 20 {
                 if temp > 30 { "Very hot!" } else { "Hot" }
             } else if temp < 10 {
                 "Cold!"
             } else {
                 "Ok!"
             }
    );
}