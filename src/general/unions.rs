#![allow(unused_variables)]

// 32 bits
union IntOrFloat {
    i: i32,
    f: f32,
}

fn process_value(iof: IntOrFloat) {
    println!("\n>> {}\n", "Unions");

    unsafe {
        match iof {
            IntOrFloat {
                i: 42
            } => {
                println!("Meaning of life value");
            }

            IntOrFloat {
                f
            } => {
                println!("Value = {}", f);
            }
        }
    }
}

pub fn unions() {
    process_value(IntOrFloat { f: 5.1 });
}