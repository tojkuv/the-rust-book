use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(self: &Self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    
    fn most_stocked(self: &Self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn add_one_v1(x: u32) -> u32 {
    x + 1
}

// // defined in the standard library
// impl<T> Option<T> {
//     pub fn unwrap_or_else<F>(self: Self, f: F) -> T 
//     where
//         F: FnOnce() -> T
//     {
//         match self {
//             Option::Some(x) => x,
//             Option::None => f(),
//         }
//     }
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// // the `Iterator` trait is defined in the standard library
// pub trait Iterator {
//     type Item;

//     fn next(self: &mut Self) -> Option<Self::Item>;

//     // methods with default implementations elided/not_shown
// }

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_preference1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_preference1);

    println!("The user with preference {:?} gets {:?}", user_preference1, giveaway1);

    let user_preference2 = None;
    let giveaway2 = store.giveaway(user_preference2);

    println!("The user with preference {:?} gets {:?}", user_preference2, giveaway2);


    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };


    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x| { x + 1 }; // compile-time error: type annotation needed (since it is not infered)
    // let add_one_v4 = |x| x + 1; // compile-time error: type annotation needed (since it is not infered)
    

    let example_closure = |x| x;
    
    let s = example_closure(String::from("hello"));
    // let n = example_closure(5); // compile-time error: mismmatched types (closure parameter inferenced as a String) 


    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // this closure captures an immutable reference
    let only_borrows = || println!("From closure: {:?}", list);
    
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);


    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // this closure captures an mutable reference
    let mut borrows_mutably = || list.push(7);

    // println!("This will not compile since it tries to borrow {:?}", list); // compile-time error: lifetimes
    
    borrows_mutably();
    println!("After calling closure: {:?}", list);


    let mut list = [
        Rectangle { width: 10, height: 1},
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // // `sort_by_key` calls the closure multiple times, once for each item in the slice.
    // list.sort_by_key(|r| r.width);
    // println!("{:#?}", list);

    // let mut sort_operations = vec![];
    // let value = String::from("sort by key was called");

    // // the closure passed to the method only implements `FnOnce` so this method call will not compile since 
    // // `sort_by_key` only takes closures that also implement the `FnMut` function trait
    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });
    // println!("{:#?}", list);

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);



    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

// note: closures capture the variables in the scope that the closures are defined in and pass the variables to the 
// code in the closure (functions are mot able to capture their enviroment in this way). the compiler determines if the
// variable should be captured as an immutable reference, mutable reference, or by taking ownership
// note: evaluation of closures with no type annotations is required since rust is a statically typed at runtime
// note: the scope braces of closures are optional if the closure's body only contains one expression

// question: why does a closure that borrows mutably need to be defined as a mutable closure?