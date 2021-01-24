#![allow(unused_variables)]

fn sum_and_product(x: i8, y: i8) -> (i16, i16) {
    ((x + y).into(), (x * y).into())
}

pub fn tuples() {
    println!("\n>> {}\n", "Tuples");

    let x: i8 = 3;
    let y: i8 = 4;

    let sp: (i16, i16) = sum_and_product(x, y);

    println!("sp = {:?}", sp);
    println!("sp {0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    // destructuring
    let (a, b) = sp;

    println!("a = {}, b = {}", a, b);

    let sp_2: (i16, i16) = sum_and_product(4, 7);
    let combined: ((i16, i16), (i16, i16)) = (sp, sp_2);

    println!("combined = {:?}", combined);

    println!("the last el from combined = {}", (combined.1).1);

    let ((a1,b1),(a2,b2)) = combined;

    println!("a1 = {}, b1 = {}, a2 = {}, b2 = {}", a1, b1, a2, b2);

    let foo = (true, 42.9, 123);
    println!("foo = {:?}", foo);

    let meaning = (42,);
    println!("meaning = {:?}", meaning);
}