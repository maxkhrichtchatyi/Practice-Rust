#![allow(unused_variables)]

use std::collections::HashMap;

pub fn hash_maps() {
    println!("\n>> {}\n", "HashMap");

    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);
    shapes.insert("circle".into(), 0);
    println!("a square has {} sides", shapes["square".into()]);

    for (key, value) in &shapes {
        println!("key = {} has value = {}", key, value);
    }

    shapes.insert("square".into(), 1);
    println!("shapes = {:?}", shapes);

    shapes.entry("other".into()).or_insert(777);
    println!("other = {}", shapes["other".into()]);

    {
        let actual = shapes.entry("other".into()).or_insert(2);
        *actual = 10;
    }

    println!("{:?}", shapes);
}