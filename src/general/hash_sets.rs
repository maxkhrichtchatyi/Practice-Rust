#![allow(unused_variables)]

use std::collections::HashSet;

pub fn hash_sets() {
    println!("\n>> {}\n", "HashSet");

    let mut greek = HashSet::new();
    greek.insert("gamma");
    greek.insert("delta");
    println!("{:?}", greek);

    greek.insert("delta");
    println!("{:?}", greek);

    let added_vega = greek.insert("vega");
    if added_vega {
        println!("we added \"vega\"!")
    }

    let added_delta = greek.insert("delta");
    if added_delta {
        println!("we added \"delta\"!")
    }

    if !greek.contains("kappa") {
        println!("we don't have \"kappa\"!");
    }

    let removed_delta = greek.remove("delta");
    if removed_delta {
        println!("we removed \"delta\"!");
    }
    println!("{:?}", greek);

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    println!("is {:?} is a subset of {:?}? {}", _2_8, _1_10, _2_8.is_superset(&_1_10));

    // disjoint = no common elements
    println!("is {:?} is a subset of {:?}? {}", _1_5, _6_10, _1_5.is_disjoint(&_6_10));

    // union, intersection
    println!("item in either {:?} and {:?} are {:?}", _2_8, _6_10, _2_8.union(&_6_10));

    // difference
    // symmetric_difference = union - intersection
    println!("item not in either {:?} and {:?} are {:?}", _2_8, _6_10, _2_8.intersection(&_6_10));

}