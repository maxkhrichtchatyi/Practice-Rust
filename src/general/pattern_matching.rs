#![allow(unused_variables)]

fn how_many(x: i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        12 => "a dozen",
        z @ 9..=11 if (z == 9) => "lots of",
        _ if (x % 2 == 0) => "some",
        _ => "few",
    }
}

pub fn pattern_matching() {
    println!("\n>>{}\n", "Pattern matching");

    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x));
    }
}