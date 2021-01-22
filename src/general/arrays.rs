#![allow(unused_variables)]

use std;

pub fn arrays() {
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];

    println!("arr has {} elements, first is {}",
             arr.len(),
             arr[0]
    );

    arr[0] = 123;

    println!("arr[0] is {}", arr[0]);
    println!("{:?}", arr);

    if arr != [1, 2, 3, 4, 5] {
        println!("arr != [1,2,3,4,5] does not match");
    }
    if arr == [123, 2, 3, 4, 5] {
        println!("arr == [123,2,3,4,5] match");
    }

    // no implementation for `[i32; 5] == [{integer}; 6]`
    // if arr == [123, 2, 3, 4, 5, 1] {
    //     println!("arr == [123,2,3,4,5,1] does not match");
    // }

    let dyn_arr = [1u8; 10]; // dyn_arr.len() == 10

    for i in 0..dyn_arr.len() {
        println!("{}", dyn_arr[i]);
    }

    println!("dyn_arr took up {} bytes", std::mem::size_of_val(&dyn_arr));

    let mtx: [[f32; 3]; 3] = [
        [1.0, 2.0, 3.0],
        [2.0, 4.0, 6.0],
        [4.0, 8.0, 12.0],
    ];

    println!("{:?}", mtx);

    for i in 0..mtx.len() {
        for j in 0..mtx[0].len() {
            // only diagonal
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}