#[allow(unused_variables)]

pub fn scope_and_shadowing() {
    println!("\n>> {}\n", "Scope and shadowing");

    let first_variable: i32 = 100;
    println!("outside, first_variable = {}", first_variable);

    {
        let second_variable: i32 = 200;
        println!("inside, second_variable = {}", second_variable);

        let first_variable: i32 = 300;
        println!("inside, first_variable = {}", first_variable);
    }

    // println!("outside, second_variable = {}", second_variable);
}