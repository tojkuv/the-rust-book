fn main() {
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3, 4, 5]; // the `vec!` macro from the `std` module does type inference 
    let third: &i32 = &v[2];

    match v.get(2) {
        Some(&element) => println!("{}", element),
        None => (),
    }

    let first = &v[0]; // first is binded to an immutable reference of the first element in `v`
    v.push(6);  

    // println!("The first element is: {}", first);    // compile-time error: the `push` method uses a mutable reference to 
                                                    // modify the Vector, which conflicts with the immutable reference 
                                                    // `first` that is in the same scope.

    let v = vec![100, 32, 57];
    for i in v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}