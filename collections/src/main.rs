fn main() {
    // let v: Vec<i32> = Vec::new();
    // let mut v = vec![1, 2, 3, 4, 5]; // the `vec!` macro from the `std` module does type inference 
    // let third: &i32 = &v[2];

    // match v.get(2) {
    //     Some(&element) => println!("{}", element),
    //     None => (),
    // }

    // let first = &v[0]; // first is binded to an immutable reference of the first element in `v`
    // v.push(6);  

    // // println!("The first element is: {}", first);    // compile-time error: the `push` method uses a mutable reference to 
    //                                                 // modify the Vector, which conflicts with the immutable reference 
    //                                                 // `first` that is in the same scope.

    // let v = vec![100, 32, 57];
    // for i in v {
    //     println!("{}", i);
    // }

    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //     *i += 50;
    // }

    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }

    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12),
    // ];

    // let mut s = String::new();

    // let data = "initial contents";

    // let s = data.to_string();

    // // the method also works on a literal directly:
    // let s = "initial contents".to_string();

    let hello = "Здравствуйте";
    // let hello = "hello";
    let answer1 = &hello[0..2];
    // let answer2 = &hello[0..1];

    println!("{}", answer1);
    println!("{}", hello);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // the arguments of `HashMap` are set to `catch-all` so the compiler performs type inferencing at compile-time
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, field_value);

    let name = &field_name; // `field_name is retains ownership of its data since `map` was passed a reference
    // `field_value` is an invalid variable at this point, since its data ownership was moved to `map`

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // the `get` method returns a `Option<&V>`; `Some(&V)` or `None`

    for (key, value) in &scores {
        println!("{}, {}", key, value);
    }

    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    let mut scores_2 = HashMap::new();

    scores_2.insert(String::from("Blue"), 10);

    // the `or_insert` method is called on the `Entry` type value, returned from the `entry` method. `or_insert` returns
    // a mutable reference to the value for the correspoding key; if the key does not exist, the method inserts the
    // argument provided as the new value for the key in the hashmap that is referenced within the `Entry` value (enum)
    scores_2.entry(String::from("Blue")).or_insert(50);
    scores_2.entry(String::from("Yellow")).or_insert(50);

    println!("{:?}", scores_2);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count = *count + 1;
    }

    println!("{:?}", map);

    // // Given a list of integers, use a vector and return the median and the mode of the list.
    // let mut list = vec![];
    // // implement a sorting algorithm and sequentially sort the vector. what does a `HashMap` have to do with this?
    
    // let mut map = HashMap::new();

    // for val in list {
    //     let count = map.entry(val).or_insert(0);
    //     *count = *count + 1;
    // }

    let string = String::from("structure");
    // let string = String::from("apple");

    let mut map = HashMap::new();

    let mut count = 0;
    for c in string.chars() {
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            if count == 0 {
                let mut string_clone = string.clone();
                string_clone.push_str("-hay");

                map.entry(string.clone()).or_insert(string_clone);
                
                println!("{:?}", map);                
                break;
            } else {
                let mut string_clone = string[count..].to_string();
                string_clone.push_str(&("-".to_string() + &string[0..count] + "ay"));

                map.entry(string.clone()).or_insert(string_clone);

                println!("{:?}", map);
                break;
            }
        }
        count = count + 1;
    }
}

// question: what does the enumerator `Entry` look like?

// note: the default hashing function is `SipHash`; this function can be replaced by other hashing functions that
// implement the `BuildHasher`