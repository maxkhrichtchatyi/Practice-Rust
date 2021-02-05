#![allow(unused_variables)]

pub fn strings() {
    println!("\n>> {}\n", "Strings");

    // utf-8
    let s: &'static str = "hello world"; // &str = string slice
    // you can't reassign
    // s = "other";
    // you can't get a char by an index
    // let h = s[0];

    for c in s.chars() {
        println!("{}", c);
    }

    for c in s.chars().rev() {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("first char = {}", first_char);
    }

    // heap
    // String
    let mut letters = String::new();
    let mut a = 'a' as u8;

    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }

    println!("{:?}", letters);

    // &str <> String
    let u:&str = &letters;

    // concatenation
    // String + str
    let z = letters + "abc";
    println!("{:?}", z);

    let mut abc_string_from = String::from("hello world");
    let mut abc_to_string = "hello world".to_string();
    abc_string_from.remove(0);
    println!("abc_string_from = {:?}", abc_string_from);
    println!("abc_to_string = {:?}", abc_to_string);
    abc_string_from.push_str("h");
    abc_to_string.push_str("h");
}