#![allow(unused_variables)]

enum Color {
    Red,
    Green,
    Blue,
    RGBColor(u8, u8, u8),
    CMYKColor { cyan: u8, magenta: u8, yellow: u8, black: u8 },
}

pub fn enums() {
    println!("\n>> {}\n", "Enumerations");

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
        Color::RGBColor(255, 255, 255)
        | Color::CMYKColor { cyan: 0, magenta: 0, yellow: 0, black: 0 } => {
            println!("Color White");
        }
        Color::RGBColor(r, g, b) => {
            println!("RGB Color is {},{},{}", r, g, b);
        }
        _ => ()
    }
}