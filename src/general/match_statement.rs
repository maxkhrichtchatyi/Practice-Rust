#![allow(unused_variables)]

pub fn match_statement() {
    let country_code = 44;
    let county = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown",
        _ => "invalid"
    };

    println!("The country with code {} is {}",
             country_code, county
    )
}