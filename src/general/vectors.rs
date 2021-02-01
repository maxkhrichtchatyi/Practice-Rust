#![allow(unused_variables)]

pub fn vectors() {
    println!("\n>> {}\n", "Vectors");

    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);

    a.push(44);

    println!("extended a = {:?}", a);
    println!("first element a[0] = {}", a[0]);

    // isize usize
    let idx: usize = 0;
    println!("first element a[0] = {}", a[idx]);

    // Option
    match a.get(6) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("Error, no such element")
    }

    for x in &a {
        println!("{}", x);
    }

    let last_element = a.pop(); // Option
    println!("last_element = {:?}", last_element);
    println!("a = {:?}", a);

    // Iterate Some that returns Option
    // yield
    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}