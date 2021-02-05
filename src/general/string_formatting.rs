#![allow(unused_variables)]

pub fn string_formatting() {
    println!("\n>> {}\n", "String formatting");

    let name = "Max";
    let greeting = format!("Hi, I'm {}, nice to meet you!", name);
    println!("{}", greeting);

    let run = "run";
    let forest = "Forest";
    let rfr = format!("{0}, {1}, {0}!", run, forest);
    println!("{}", rfr);

    let info = format!(
        "the name's {last}. {first} {last}",
        first = "James",
        last = "Bond"
    );
    println!("{}", info);

    let mixed = format!(
        "{1} {} {0} {} {delta}",
        "alpha",
        "betta",
        // "gamma", // unused
        delta = "delta"
    );
    println!("{:?}", mixed);
}