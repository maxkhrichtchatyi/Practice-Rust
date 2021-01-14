#![allow(unused_variables)]

pub fn arrays() {
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];

    println!("arr has {} elements, first is {}",
             arr.len(),
             arr[0]
    );

    arr[0] = 123;

    println!("arr[0] is {}", arr[0]);
}