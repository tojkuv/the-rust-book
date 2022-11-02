// fn largest<T>(list: &[T]) -> T {
//     // this function is a generic function over type `T`. generic type declarations use the same convention as
//     // concrete types. A single letter is usually prefered. generic types must be decalred before the parameters of
//     // the function before they can be used in the function signature and function body.
//     // --snip--
// }

enum Option<T> {
    // the enumeration `Option` is generic over type `T`

    // the `Some` variant holds one value of type `T`
    Some(T),
    // the `None` variant does not hold any values
    None,
}

enum Result<T, E> {
    // the enumeration `Result` is generic over type `T` and type `E`

    // the `Ok` variant holds one value of type `T`
    Ok(T),
    // the `Err` variant holds one value of type `E`
    Err(E),
}

// struct Point<T> {
//     // the structure `Point` is generic over type `T` and contains the field variable `x` of type `T` and the
//     // field variable `y` of type `T`

//     // `x` and `y` are of type `T`; they can be any type, but they both must be the same type
//     x: T,
//     y: T,
// }

struct Point<X1, Y1> {
    // the structure `Point` is generic over type `X1` and some type `Y1` and contains the field variable `x` of
    // type `X1` and the field variable `y` of type `Y1`

    // `x` is of type `X1` and `y` is of type `Y1`; they can be any type and not have to be the same types
    x: X1,
    y: Y1,
}

// impl<T> Point<T> {
//     // `impl` implements generically over type `T` for the structure `Point` that is generic over type `T`. associated
//     // functions defined within this scope can use the generic types defined by `impl`. do all the types represented by
//     //`T` have to be the same concrete type?. It's convention for the generic type of `impl` to share the same parameter
//     // type name as the structure definition.
//     fn x(self: &Self) -> &T {
//         //
//         &self.x
//     }
// }

// impl Point<f32> {
//     // `impl` will implement the following associated function for the `Point` structure that has a `f32` type for the
//     // generic parameter `T`
//     fn distance_from_origin(self: &Self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self: Self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        // when does one `self: &Self` instead of `self: Self` - if the generic type can be a type that implements the
        // move trait, the parameter must have ownership of the `Self` instead of a reference to self
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

mod aggregator; // declares the module and rust will look for the module in 3 different places

use crate::aggregator::{NewsArticle, Summary, Tweet}; // imports the spesified parts of the module for performance

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("{}, {}", p3.x, p3.y);

    let integer = Some(5);
    let float = Some(5.0);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the hocket team in the NHL."),
    };

    println!("New article available! {}", article.summarize());

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result1 = largest(&char_list);
    println!("The largest number is {}", result);

    // let result;

    let string1 = String::from("long string is long");

    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    //     // the line above will compile since the shortest lifetime of the arguments lives as long as the lifetime of
    //     // the result value
    // }

    println!("{}", result);

    // advanced traits
    let person = Human;
    person.fly();

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    eprintln!("A baby dog is called a {}", Dog::baby_name());
}

// // the `integer` and `float` variables in the main method monomorphize the generic enumaration `Option` into two
// // concrete type enumerations derived from `Option` at compile-time to have zero-cost abstractions at run-time.
// // the compiled `Option`enumeration gets transformed into the following two enumeration types:

// enum Option_i32 {
//     Some(i32),
//     None,
// }

// enum Option_f64 {
//     Some(f64),
//     None,
// }

// fn largest<T: PartialOrd + Clone>(list: &[T]) -> T {
//     let mut largest = list[0].clone();

//     for item in list {
//         if *item > largest {
//             largest = item.clone(); // copy not move so its ok
//         }
//     }

//     largest
// }

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        // item is an immutable reference to an element in list
        if item > largest {
            // automatic referencing is done by the compiler
            largest = item;
        }
    }

    largest
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

pub trait Animal {
    fn baby_name() -> String;
}

pub struct Dog;

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

impl Dog {
    fn baby_name() -> String {
        String::from("Sofie")
    }
}
