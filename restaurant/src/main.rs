use std::collections::HashMap; // this is standard practice but i won't follow it at first.
// use std::collections;

fn main() {
    let mut map = HashMap::new();
    // let mut map = collections::HashMap::new();
    map.insert(1, 2);
}

use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}