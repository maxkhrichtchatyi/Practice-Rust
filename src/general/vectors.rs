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
}