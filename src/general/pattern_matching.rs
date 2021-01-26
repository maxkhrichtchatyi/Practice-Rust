#![allow(unused_variables)]

fn how_many(x: i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        12 => "a dozen",
        z @ 9..=11 if (z == 9) => "lots of",
        _ if (x % 2 == 0) => "some",
        _ => "few",
    }
}

enum Color {
    Red,
    Green,
    Blue,
    RGBColor(u8, u8, u8),
    CMYKColor { cyan: u8, magenta: u8, yellow: u8, black: u8 },
}

pub fn pattern_matching() {
    println!("\n>>{}\n", "Pattern matching");

    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x));
    }

    let point = (3, 4);

    match point {
        (0, 0) => println!("origin"),
        (0, y) => println!("x axis, y = {}", y),
        (x, 0) => println!("x = {}, y axis", x),
        (x, y) => println!("x = {}, y = {}", x, y)
    }

    let color_regular: Color = Color::Red;
    let color_rgb: Color = Color::RGBColor(255, 255, 255);
    let color_cmyk: Color = Color::CMYKColor {
        cyan: 0,
        magenta: 0,
        yellow: 0,
        black: 0,
    };

    match color_cmyk {
        Color::Red => {
            println!("Color Red");
        }
        Color::Blue => {
            println!("Color Blue");
        }
        Color::Green => {
            println!("Color Green");
        }
        Color::RGBColor(255, 0, 0)
        | Color::CMYKColor { black: 0, .. } => {
            println!("Color Black");
        }
        Color::RGBColor(r, g, b) => {
            println!("RGB Color is {},{},{}", r, g, b);
        }
        _ => ()
    }
}
