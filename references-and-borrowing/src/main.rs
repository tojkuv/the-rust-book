// the scope of a reference starts at the declaration of the reference until the last time it is used
    // there can be multiple immutable references in the same scope
    // if there is a mutable reference, it is the only reference that is allowed to be declared within the references'
    // lifetime

fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1);

    // let r1 = &mut s1;
    // let r2 = &mut s1;

    // println!("{}, {}", r1, r2); //compiler-time error: more than one mutable reference is in scope when r1 is used

    {
        // let r3 = &mut s1;
    } // r3 goes out of scope

    // let r4 = &mut s1;

    let r5 = &s1; // no error or warning
    let r6 = &s1; // no error or warning
    
    // let r7 = &mut s1; // error: cannot borrow 's1' as mutable because it is also borrowed as immutable
    
    println!("{}, {}", r5, r6);
    // variables r1 and r2 will not be used after this point
    
    let r7 = &s1;

    println!("{}", r7);
    // variable r7 falls out of scope here

    let reference_to_nothing = dangle(); // dangling reference
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
}   // Here, s goes out of scope, but because it does not have ownership of the String, the value of the String is not
    // dropped

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String { // dangle returns a reference to a string
    let s = String::from("hello"); // s is a new String

    s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away
