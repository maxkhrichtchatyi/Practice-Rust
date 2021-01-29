#![allow(unused_variables)]

// Option<T>
struct Point<T, V> {
    x: T,
    y: V,
}

struct Line<T, V> {
    start: Point<T, V>,
    end: Point<T, V>,
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

    let point_line_a: Point<f32, f32> = Point {
        x: 3.0,
        y: 4.0,
    };

    let point_line_b: Point<f32, f32> = Point {
        x: 13f32,
        y: 14.0,
    };

    let my_line = Line {
        start: point_line_a,
        end: point_line_b,
    };

    println!("my_line start x={} y={}, end x={} y={}", my_line.start.x, my_line.start.y, my_line.end.x, my_line.end.y);
}