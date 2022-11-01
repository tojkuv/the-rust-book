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

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(*)
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
        // person.fly();

        Pilot::fly(&person);
        Wizard::fly(&person);
        // person.fly();

        eprintln!("A baby dog is called a {}", Dog::baby_name());
    }
}

// question: if one imports an external struct that implements multiple methods with the same method signature, why must one
// specify the scope of the method implemented directly into the object, unlike an object that was declared locally?
