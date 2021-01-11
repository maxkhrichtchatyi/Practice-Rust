#![allow(unused_variables)]

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

pub fn structs() {
    println!("\n>> {}\n", "Structs");

    let point = Point {
        x: 10.5,
        y: 10.5,
    };

    println!("Point is at {} {}", point.x, point.y);

    let other_point = Point {
        x: 5.0,
        y: 3.0,
    };

    println!("Other point is at {} {}", other_point.x, other_point.y);

    let line = Line {
        start: point,
        end: other_point,
    };

    println!("Line start x:{}, y:{}, end x:{}, y:{}", line.start.x, line.start.y, line.end.x, line.end.y);
}