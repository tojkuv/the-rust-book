#[cfg(test)]
mod tests {
    #[test]
    fn match_expressions() {
        let x = Some(1);

        let _y = match x {
            None => None,
            Some(i) => Some(i + 1),
        };
    }

    #[test]
    fn if_expressions() {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();
    
        if let Some(color) = favorite_color {
            // if expression do not enforce exhaustiveness like match expression do
            println!("Using your favorite color, {color}, as the background");
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            // the variable age is shadowed by the pattern's age variable
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }

    #[test]
    fn while_loop_expressions() {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);
    
        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }

    #[test]
    fn for_loop_expressions() {
        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            // the tuple pattern captures the destructured iteration. the for loop implements the
            // logic at compile time.
            println!("{} is at index {}", value, index);
        }
    }

    #[test]
    fn let_statements() {
        // // the structure of a `let` statement; bind the evaluation of the expression to the
        // // pattern. the structure of the pattern must match the structure of the value from the
        // // expression
        // let PATTERN = EXPRESSION;

        // using a pattern to destructure a tuple and create three variables in one statement
        let (_x, _y, _z) = (1, 2, 3);

        // // compile-time error: the values do not match the pattern
        // let (x, y) = (1, 2, 3);

    }

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
    
    #[test]
    fn function_parameters() {
        let point = (3, 5);
        print_coordinates(&point);
    }
    
    fn print_rgb<F>(_f: F)
    where
        F: Fn(i32, i32, i32) -> ()
    {
        let _r = 123;
        let _g = 23;
        let _b = 231;
    }

    #[test]
    fn closure_parameters() {
        let closure = |r, g, b| {
            println!("the RGB colors are: ({}, {}, {})", r, g, b);
        };

        print_rgb(closure);
    }

    #[test]
    fn irrefutable_patterns() {
        // irrefutable patterns will match for any possible value passed
        // the `let` statement can only accept irrefutable patterns!
        let x = 5;

        // note: function parameters, `let` statements, and `for` loops can only accept
        // irrefutable patterns
        
        // using an irrefutable pattern in an `if let` expression cause the compiler to create a
        // warning since binding will never fail
        if let x = 5 {
            println!("{}", x);
        };

        // using a single irrefutable pattern in a match expression is possible; it is not very
        // useful. it is more practical to use an `let` statement
        match 1 {
            g => println!("{}", g),
        }
    }

    #[test]
    fn refutable_patterns() {
        // a_value is an irrefutable
        let a_value = Some(1);
        
        // refutable patterns can fail to match a possible value
        if let Some(x) = a_value {
            println!("{}", x);
        }

        // note: `if let` and `while let` expression accept irrefutable and refutable patterns;
        
        // note: the compiler will create a warning when it finds an irrefutable pattern in
        // `if let` and `while let` expressions since the expressions are meant to have failure
        // cases

        let some_option_value: Option<i32> = None;
        // // compile-time error: `Some(x)` is a refutable pattern and the `let` statement only
        // // accepts an irrefutable pattern
        // let Some(x) = some_option_value;

        // using `if let` and a block with refutable patterns instead of `let`
        if let Some(x) = some_option_value {
            // the `Some(x)` pattern is refutable. To avoid a compile-time error, we must define
            // a scope that will only be run if the binding to the refutable pattern does not fail
            println!("{}", x);
        }

        // note: by design, match arm patterns must use refutable patterns except for the last arm,
        // which should match any remaining values, if there are multiple remaining, with an
        // irrefutable pattern
    }

    #[test]
    fn matching_literals() {
        let x = 1;

        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }

        let x = Some(5);
        let y = 10;

        match x {
            // the `literal in the arm `Some()` does not match
            Some(50) => println!("Got 50"),
            // the irrefutable pattern `y` matches (y is a new shadowing variable that binds to 5) 
            Some(y) => println!("Matched, y = {y}"),
            _ => println!("Default case, x = {:?}", x),

        }
    }

    #[test]
    fn matching_multiple_patterns() {
        let x = 1;

        match x {
            // `|` is the `or` operator for patterns
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    #[test]
    fn matching_ranges_of_values() {
        let x = 5;

        match x {
            1..=5 => println!("one through five"),
            _ => println!("something else"),
        }

        let y = 'c';

        match y {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }

        // note: the compiler checks that the range isn't empty at compile-time. Because the only
        // types for which the compiler can tell if a range is empty or not are `char` and 
        // numeric values, ranges are only allowed with numeric or `char` values.
    }

    struct Point2D {
        y: i32,
        x: i32,
    }

    #[test]
    fn destructuring_structs() {
            let p = Point2D { x: 0, y: 7 };

        // destructuring struct fields into separate variables
        let Point2D { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);

        // destructuring struct fields using struct field shorthand
        let Point2D {x, y} = p;
        assert_eq!(0, x);
        assert_eq!(7, y);

        match p {
            Point2D { x, y: 0 } => println!("On the x axis at {}", x),
            Point2D { x: 0, y } => println!("On the y axis at {}", y),
            Point2D { x, y } => println!("On neither axis: ({}, {})", x, y),
        }

        // note: destructuring field variables with patterns requires that one specifies the name
        // of the field variables (1 to 1 correspondence of variable names and field names)
    }

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    #[test]
    fn destructuring_enums() {
        // let msg = Message::ChangeColor(0, 160, 255);
        
        // match msg {
        //     Message::Quit => {
        //         println!("The Quit variant has no data to destructure.")
        //     }
        //     Message::Move { x, y } => {
        //         println!("Move in the x direction {} and in the y direction {}", x, y);
        //     }
        //     Message::Write(text) => println!("Text message: {}", text),
        //     Message::ChangeColor(r, g, b) => println!(
        //         "Change the color to read {}, green {}, and blue {}", r, g, b),
        // }


        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
                "Change the color to red {}, green {}, and blue {}", r, g, b
            ),
            Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
                "Change the color to hue {}, saturation {}, and value {}", h, s, v
            ),
            _ => (),
        }
    }

    #[test]
    fn destructuring_complex_nested_values() {
        // creating a getter method for the enum is a more sound solution
        if let ((_feet, _inches), Point2D { x: _x1, y: _y1 }, Point2D { x: _x2, y: _y2 }, Message::Write(_t))
            = ((3, 10), Point2D { x: 3, y: -10 }, Point2D { x: -6, y: 12 },
            Message::Write("text".to_string())) {};
    }

    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    #[test]
    fn ignoring_entire_values() {
        // the `foo()` function ignore the first parameter using the `_` catch-all pattern
        foo(3, 4);
    }

    #[test]
    fn ignoring_parts_of_a_value() {
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            // this match arm does not capture the associated data of either Some value
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }
            _ => {
                setting_value = new_setting_value;
            }
        }

        println!("setting is {:?}", setting_value);
        
        
        let numbers = (2, 4, 8, 16, 32);
        
        match numbers {
            // this arm ignores multiple parts of a tuple
            (first, _, third, _, fifth) => {
                println!("Some numbers: {first}, {third}, {fifth}")
            }
        }
        
        // the compiler will not create an unused variable warning for the variable `x` since it's
        // prefixed with the catch-all pattern
        let _x = 5;
        let y = 10;


        
        
        let s = Some(String::from("Hello!"));
        // note: the syntax `_x` still binds the value to the variable, whereas `_` doesn't bind 
        // at all

        if let Some(_s) = s {
            println!("found a string");
        }

        // // compile-time error: `String` implements the copy trait so the value is moved from `s`
        // println!("{:?}", s);

        let s = Some(String::from("Hello!"));

        if let Some(_) = s {
            println!("found a string");
        }

        // using catch-all in the expression does not bind any data away form `s`
        println!("{:?}", s);
    }

    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    #[test]
    fn ignoring_remaining_parts_of_a_value() {
        let origin = Point3D { x: 0, y: 0, z: 0 };

        match origin {
            // ignoring all fields of a `Point` except for `x` by using `..`
            Point3D { x, .. } => println!("x is {}", x),
        }


        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            // ignoring all th1.wbe tuple values between the first value and the last value
            (first, .., last) => {
                println!("Some numbers: {first}, {last}");
            }
        }

        // match numbers {
        //     // compile-time error: `..` cannot be used ambiguously (only one `..` per tuple)
        //     (.., second, ..) => {
        //         println!("Some numbers: {second}");
        //     }
        // }
    }

    #[test]
    fn match_guards() {
        let num = Some(4);

        match num {
            // the match guard `if x % 2 == 0` is conditional statement that all needs to be
            // satisfied to match with the specified arm
            Some(x) if x % 2 == 0 => println!("The number {} is even", x),
            Some(x) => println!("The number {} is odd", x),
            None => (),
        }

        // note: the downside of this additional expressiveness is that the compiler doesn't try
        // to check for exhaustiveness when match guard expressions are involved

        
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("Matched, n = {n}"),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {y}", x);

        let x = 4;
        let y = false;

        match x {
            // combining multiple patterns with a match guard
            // precedence: (4 | 5 | 6) if y
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }
    }

    #[test]
    fn field_value_bindings() {
        let msg = Message::Hello { id: 5 };

        match msg {
            // the `@` operator to binds the field value to a new variable while the value is
            // evaluated with a match guard
            Message::Hello { id: id_variable @ 3..=7 } => {
                println!("Found an id in range: {}", id_variable)
            }
            // the field value is evaluated  with a match guard, but the value cannot be used in
            // code that goes with the pattern
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            // the field value can be used with the code that goes with the pattern, but it
            // cannot be evaluated with match guards
            Message::Hellso { id } => println!("Found some other id: {}", id),
        }

        println!("hello world");
    }
}

// note: a variable name is a pattern. the compiler compares t`he expression with the pattern and
// assigns any names it finds.
