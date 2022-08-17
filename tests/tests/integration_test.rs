// use creates a symbolic link to the location of the specified item. this item can be used in this scope without 
// specifying the entire path. an item's module must be imported into this scope before a symbolic link can be created
use tests; 

// `mod` includes public api code from the specified module into this scope
mod common; 

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, tests::add_two(2));
}

// only library crates expose functions. binary crates are not meant to expose functions; integration tests cannot
// test functions of binary crates; if a project only contains a binary crate, the compiler cannot generate integration
// tests

// This is one of the reasons Rust projects that provide a binary have a straightforward _src/main.rs_ file that calls
// logic that lives in the _src/lib.rs_ file. Using that structure, integration tests _can_ test the library crate
// with `use` to make the important functionality available. If the important functionality works, the small amount of
// code in the _src/main.rs_ file will work as well, and that small amount of code doesn't need to be tested.