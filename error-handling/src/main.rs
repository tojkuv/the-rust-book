use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // // this panic macro call stop the program and creates an error with the following message and the location of the
    // // macro call. in other cases the call might be execured in a dependency and thus the error will refer to its
    // // location in the dependency code
    // panic!("crash and burn");

    let v = vec![1, 2, 3];

    // // run-time error: calls the `panic!` macro
    // v[99];

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    use::std::fs::File; // `File` is a file handle

    // // the `Result` enumarator is brought into scope by the prelude
    // let file = File::open("hello.txt");
    // let file = match file {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    use::std::io::ErrorKind;

    let f = File::open("hello_1.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello_1.txt") {
                Ok(file) => file,
                Err(error) => panic!("Problem creating the file: {:?}", error),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    let f = File::open("hello_2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello_2.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // let file = File::open("hello_3.txt").unwrap();
    // // the `expect` associated function of the `Result` enumeration has the same functionality as the `unwrap`
    // // associated function with a string slice parameter that is passed to the `panic!` macro; the `expect`
    // // associated function can be used to provide more verbose error messages when needed.
    // let file = File::open("hello_3.txt").expect("Failed to open hello.txt");

    use std::io::{self, Read};

    fn read_username_from_file_error_propagation_match_conditionals() -> Result<String, io::Error> {
        // the generic type `T` from result has been filled in with the concrete type `String` and the generic type
        // `E` has been filled in with the concrete type `io::Error`. this function implements the concept of
        // error propagation, where the potential errors of the function are passed to the calling code to handle the
        // errors. propagating all the success and error cases to the calling code is done so the calling code can
        // handle both cases apropriately for it's utility
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    fn read_username_from_file_with_error_propagation_operator() -> Result<String, io::Error> {
        // the `?` error propagation operator is used on the `Return` enumerator for error handling. 
        // the `?` error propagation operator returns the associated data of the `Ok` variant of the `Return`
        // enumeration type or if the value is an `Err` variant, the `Err` variant gets returned from the scope from
        // which the error propagation operator is called from, very much like using the `return` keyword on the `Err`
        // variant
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();

        f.read_to_string(&mut s)?;

        Ok(s)
    }

    fn read_username_from_file_with_error_propagation_operator_compressed() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }

    use std::fs;

    fn read_username_from_file_with_associated_function() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    // the following line creates a compile-time error since the `main` function has an empty tuple as the return type;
    // the error propagation operator `?` can only be called in functions that return a concrete type that implements
    // the trait `FromResidual<Result<Infallible, std::io::Error>>`.
    // let f = File::open("hello.txt")?;

    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }

    // since the main function return a `Result` variant, the error propigation operator `?` can be used in it's scope
    let f = File::open("hello_2.txt")?;

    // loop {
    //     // --snip--

    //     let guess: i32 = match guess.trim().parse() {
    //         // first checks if the input guess in a number
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     if guess < 1 || guess > 100 {
    //         // then checks if the number is not in the correct range
    //         println!("The secret number will be between 1 and 100.");
    //         continue;
    //     }

    //     match guess.cmp(&secret_number) {
    //         // a match conditional handling all the game logic
    //         // --snip--
    //     }
    // }

    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            // the `new` constructor 
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess { value }
        }

        pub fn value(self: &Self) -> i32 {
            self.value
        }
    }

    // if no errors are propigated back to the main function, the following line will return a `Result` variant that
    // will contain an empty tuple
    Ok(())
}

// note: `unwrap` and `expect` are practical for prototyping
// note: the `?` error propagation operator converts the associated data of whatever error type inside the `Err`
// variant of Result into the associated data of whatever the error type of the current scope's function is set to,
// as long as the error type conversion is implemented in the `from` function of the `From` trait

// question: what is a file handle?