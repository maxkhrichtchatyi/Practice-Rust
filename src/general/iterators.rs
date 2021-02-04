#![allow(unused_variables)]

pub fn iterators() {
    println!("\n>> {}\n", "Iterators");

    let vec = vec![3, 2, 1];

    for x in &vec {
        println!("{}", *x);
    }

    for x in vec.iter() {
        println!("iter {}", x);
    }
}