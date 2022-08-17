#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn can_hold(self: &Self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[derive(Debug)]
pub struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }
        
        if value > 100 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        }

        Guess { value }
    }
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn greeting(name: &str) -> String {
    format!("Hello!")
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}


// this is a unit test since the tests are run within the module that is being tested
#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() -> Result<(), String> {
    //     if 2 + 2 == 4 {
    //         Ok(())
    //     } else {
    //         Err(String::from("two plut two does not equal four"))
    //     }
    // }

    // #[test]
    // // the [should_panic] attribute annotates the function as a function that should call the `panic!` for the test to
    // // pass. if the test function does not call the `panic!` the test will fail.
    // // the `should_panic` attribute can have the _tag_ `expected set to a string slice. the function should call a
    // // `panic!` macro with a custom message that contains the string slice specified by the expected tag
    // #[should_panic(expected = "Guess value must be less than or equal to 100")]
    // fn greater_than_100() {
    //     Guess::new(200);
    // }
    
    // #[test]
    // fn greeting_contains_name() {
    //     let result = greeting("Carol");
    //     assert!(result.contains("Carol"), "Greeting did not contain name, value was `{}`", result);
    // }

    // #[test]
    // fn it_adds_two() {
    //     assert_eq!(4, add_two(2));
    // }

    // #[test]
    // fn larger_can_hold_smaller() {
    //     let larger = Rectangle {
    //         width: 8,
    //         height: 7,
    //     };
    
    //     let smaller = Rectangle {
    //         width: 5,
    //         height: 1,
    //     };

    //     assert!(larger.can_hold(&smaller));
    // }
    
    // #[test]
    // fn smaller_cannot_hold_larger() {
    //     let larger = Rectangle {
    //         width: 8,
    //         height: 7,
    //     };
    
    //     let smaller = Rectangle {
    //         width: 5,
    //         height: 1,
    //     };

    //     assert!(!smaller.can_hold(&larger));
    // }

    // #[test]
    // fn this_test_will_pass() {
    //     let value = prints_and_returns_10(4);
    //     assert_eq!(10, value);
    // }

    // #[test]
    // fn this_test_will_fail() {
    //     let value = prints_and_returns_10(8);
    //     assert_eq!(5, value);
    // }

    // #[test]
    // fn add_two_and_two() {
    //     assert_eq!(4, add_two(2));
    // }

    // #[test]
    // fn add_three_and_two() {
    //     assert_eq!(5, add_two(3));
    // }

    // #[test]
    // fn one_hundred() {
    //     assert_eq!(102, add_two(100));
    // }

    // #[test]
    // #[ignore]
    // fn expensive_test() {
    //     // code that takes an hour to run
    // }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

// question: if the unit tests fail, will the integration tests run?
    // assumption: no, because the compile assumes the module is not working as it was intended, thus the integration
    // tests will not test as intended.