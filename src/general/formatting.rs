#![allow(unused_variables)]

pub fn fmt() {
    let a: f64 = 10.0;
    let b: f32 = 3.0;
    let c: f64 = a / b as f64;

    println!("c is {:.3}", c);
    println!("c is {:8.3}", c);
    println!("c is {:08.3} \na is {1}\nonce again {0}", c, a);
}