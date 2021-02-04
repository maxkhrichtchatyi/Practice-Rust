#![allow(unused_variables)]

pub fn iterators() {
    println!("\n>> {}\n", "Iterators");

    let mut vec = vec![3, 2, 1];

    for x in &vec {
        println!("{}", *x);
    }

    for x in vec.iter() {
        println!("iter {}", x);
    }

    for x in vec.iter_mut() {
        *x += 1;
    }

    println!("vec = {:?}", vec);

    for x in vec.iter().rev() {
        println!("rev iter {}", x);
    }

    let mut vec_two = vec![1,2,3];
    vec_two.extend(vec);
    println!("vec_two = {:?}", vec_two);
}