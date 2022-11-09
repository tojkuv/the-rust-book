mod traits {
    use std::fmt;
    use std::fmt::Formatter;
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Self::Output {
            Millimeters(self.0 + (other.0 * 1_000))
        }
    }

    pub trait Pilot {
        fn fly(&self);
    }

    pub trait Wizard {
        fn fly(&self);
    }

    pub struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            eprintln!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            eprintln!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            eprintln!("*waving arms furiously*");
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

    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();

            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl OutlinePrint for Point {}

    /* a wrapper is simply a struct that contains another struct that is not defined in the module.
    this can be used to implement external traits on external types. */
    pub struct VecWrapper(pub Vec<String>);

    /* the `new-type` pattern is a lightweight way to achieve encapsulation to hide implementation
     details. */
    impl fmt::Display for VecWrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    /* alias direct synonym. */
    type Kilometers = i32;

    fn takes_a_long_time(f: Box<dyn Fn() + Send + 'static>) {
        // --snip--
    }

    fn returns_a_long_type() -> Box<dyn Fn() + Send + 'static> {
        // --snip--

        return Box::new(|| println!())
    }

}


#[cfg(test)]
mod tests {
    use crate::traits::*;

    #[test]
    fn operator_overloading() {
         assert_eq!(
             Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
             Point { x: 3, y: 3}
         );
    }

    #[test]
    fn methods_with_the_same_name() {
        let person = Human;
        Pilot::fly(&person);
        Wizard::fly(&person);
        // person.fly();

        eprintln!("A baby dog is called a {}", Dog::baby_name());
    }

    #[test]
    fn wrapper() {
        /* `new-type` declarations do not have the methods of the value it holds. implementing the
        `Deref` trait on the `new-type` wrapper would allow the compiler to dereference the wrapper
        to access the inner value when calling the value's method. to restrict the wrappers type's
        behavior one can also implement only the methods that one desires to be callable from the
        wrapper. */
        let w = VecWrapper(vec![String::from("hello"), String::from("sofie")]);
        eprintln!("w = {}", w);
    }

    #[test]
    fn alias() {
        type Kilometers = i32;

        let x: i32 = 5;
        let y: Kilometers = 5;

        /* there is no type checking for aliases if we mix up aliases with their aliasing values
        when unintended the compiler will not know the difference */
        eprintln!("x + y = {}", x + y);
        assert_eq!(x, y);


        let lengthy_type: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi sofie!"));


        type Thunk = Box<dyn Fn() + Send + 'static>;
        let f: Thunk = Box::new(|| println!("hi sofie!"));

        fn takes_long_time(f: Thunk) {
            // --snip--
        }

        fn returns_long_type() -> Thunk {
            Box::new(|| println!("hi sofie!"))
        }

        type Result<T> = std::result::Result<T, std::io::Error>;
    }

    #[test]
    fn never_type() {
        // /* the `never type` is denoted as an `!`. diverging functions do not
        //  return at any point (not even an empty tuple) */
        // fn bar() -> ! {
        //     // --snip--
        // }

        // /* the `continue` expression evaluates to a never type, so the type
        //  checking for u32 is valid to the compiler; nothing is assigned to
        //  `guess` in the `Err` match arm */
        // let guess: u32 = match guess.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => continue,
        // };

        /* note: expressions of type `!` can be coerced into any other type */

        // /* since the `panic` macro evaluates to a never type, the only time a
        //  value is returned is when the `Option` value is `Some`, thus there is
        //  only one return type for the function, which is means the function
        //  will compile */
        // impl<T> Option<T> {
        //     pub fn unwrap(self) -> T {
        //         match self {
        //             Some(val) => val,
        //             None => panic!("called `Option::unwrap()` on a `None` value"),
        //         }
        //     }
        // }

        // /* loops that don't terminate also evaluate to the never type, since
        //  they are a divergent process */
        // print!("forever ");
        // loop {
        //     print!("and ever ");
        // }
    }

    #[test]
    fn dynamically_sized_types() {
        /* note: all values of the same type must have the same amount of memory
         allocated to them by the compiler */

        /* `str` is a dynamically sized type. since the size of `str` can
         changed at runtime, the final size of the `str` value cannot be
         inferred at compile-time, so the compiler cannot allocate memory for
         an `str` variable, thus creating an `str` variable is not allowed by
         the compiler */
        let s1: str = "hello sofia!";
        let s2: str = "how's it going?";

        /* note: a reference to a `str` is two values (the pointer to the first
        byte and the amounts of bytes that are allocated to the `str` value */
    }
}

// question: if one imports an external struct that implements multiple methods with the same method signature, why must one
// specify the scope of the method implemented directly into the object, unlike an object that was declared locally?
