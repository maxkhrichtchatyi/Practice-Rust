#![allow(unused_variables)]

pub fn for_loop() {
    for item in 1..11 {
        if item == 3 {
            continue;
        }

        if item == 9 {
            break;
        }

        println!("Item {}", item);
    }

    for (pos, item) in (1..11).enumerate() {
        println!("Pos {} Item {}", pos, item);
    }
}