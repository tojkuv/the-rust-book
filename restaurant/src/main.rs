use std::collections::HashMap; // this is standard practice but i won't follow it at first.
// use std::collections;
use rand::Rng;

fn main() {
    let mut map = HashMap::new();
    // let mut map = collections::HashMap::new();
    map.insert(1, 2);
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
}

// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }

// use std::{cmp::Ordering, io};   // scopes can be defined with a super path that every module in the scope inherits.
//                                 // `cmp::Ordering` and `io` modules inherit the super path `std` at compile-time.

// use std::io;    // there is a more concise way to writing this line and the line that follows in one line using inherited
//                 // paths
// use std::io::Write;

use std::io::{self, Write}; // the `self` keyword can be used in path scopes to also import the super module of the
                            // rested scope