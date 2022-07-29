fn main() {
    let s = "hello";

    let s = "hello";

    let s = String::from("hello");

    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    let s = String::from("hello");

    let x = 5;
    let y = 6;

    let s1 = String::from("hello");
    let s2 = s1;

    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1); //error: s1 does not hold ownership of the "hello" String anymore

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s = String::from("hello");  // s comes inton scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterwards

    let h1 = gives_ownership();             // gives_ownership moves its return
                                            // value into h1

    
    let h2 = String::from("hello");         // h2 comes into scope

    let h3 = takes_and_gives_back(h2);      // h2 is moved into 
                                            // takes_and_gives_back, which also
                                            // moves its return value into h3   

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}   // Here, x goes out of scope, then s. But because s's value was moved, nothing
    // special happens. Here, h3 goes out of scope and is dropped.s2 was moved, so
    // nothing happens. h1 goes out of scope and is dropped.

fn takes_ownership(some_string: String) {   // some_string comes into scope
    println!("{}", some_string);
}   // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}   // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {            // gives_ownership will move its
                                            // return value into the function
                                            // that calls it
    let some_string = String::from("yours"); // some_string comes into scope

    some_string                             // some_string is returned and
                                            // moves out to the calling
                                            // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {   // a_string comes into
                                                        // scope
    a_string   // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();   // len() returns the length of a String

    (s, length)
}