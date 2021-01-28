#![allow(unused_variables)]

// Option<T>
struct Point<T, V> {
    x: T,
    y: V,
}

pub fn generics() {
    println!("\n>>{}\n", "Generics");

    let a: Point<i16, i16> = Point {
        x: 0,
        y: 0,
    };

    let b: Point<i16, f32> = Point {
        x: 3,
        y: 4.1,
    };

    let c = Point {
        x: true,
        y: false,
    };

    println!("a x={} y={}", a.x, a.y);
    println!("b x={} y={}", b.x, b.y);
    println!("c x={} y={}", c.x, c.y);
}