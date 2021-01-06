#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;

struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

pub fn stack_and_heap() {
    println!("\n>> {}\n", "Stack and heap");

    let point_first: Point = origin();
    let point_second: Box<Point> = Box::new(origin());

    println!("point_first takes up {} bytes", mem::size_of_val(&point_first));
    println!("point_second takes up {} bytes", mem::size_of_val(&point_second));

    let get_from_the_box: Point = *point_second;
    println!("get_from_the_box Point.x = {} Point.y = {}", get_from_the_box.x, get_from_the_box.y);
}