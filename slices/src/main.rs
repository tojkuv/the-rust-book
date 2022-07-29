fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value s

    s.clear(); // compile-time error

    println!("the first word is: {}", word);

    // word still has the value 5 here, but there's no more string that we could meaningfully use the value 5 with.
    // word is now without context

    let s = String::from("hello world");

    let hello = &s[0..5];           // starting_index: 0, ending_index: 5, length: 5 = 5 - 0
    let hello = &s[..5];            // this is the same as above
    let world = &s[6..11];          // starting_index: 6, ending_index: 11, length: 5 = 11 - 6
    let world = &s[6..];            // this is the same as above

    let hello_world = &s[0..11];    
    let hello_world = &s[..];       // this is the same as above

    let s = "Hello, world!";        // string literals are slices that point to immutable data in the binary

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    let word = first_word(&my_string); // equivalent to the line above

    let first_string_literal = "hello word";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literal *are* string slices already, this works too, without the slice syntax
    let word = first_word(&my_string_literal);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {    // using &str as the type for the parameter s, allows the function to take
                                    // Strings and slices (string literals are also slices)
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];        // the compiller ensures that the reference to the string remains valid
                                    // by checking for dangling references  
        }
    }

    &s[..]
}
