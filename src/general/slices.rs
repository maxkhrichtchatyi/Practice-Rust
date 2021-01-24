#![allow(unused_variables)]

fn use_slice(slice: &mut [i8]) {
    println!("first element = {}, len = {}", slice[0], slice.len());
    slice[0] = 100;
}

pub fn slices() {
    println!("\n>> {}\n", "Slices");

    let mut arr: [i8; 4] = [1, 2, 3, 4];

    use_slice(&mut arr[1..3]);
    // use_slice(&mut arr);
    println!("arr = {:?}", arr);
}