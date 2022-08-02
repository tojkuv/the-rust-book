fn main() {
    // `match` compares a value against a series of patterns and then executes the statements and expressions
    // within the scope of the matched pattern. patterns can be made up of literal values, variables, and wildcards (?),
    // et cetera. all possible cases must be handled by the patterns

    // match (expression) {
    //     // the expression can return any type that will be used to match to a pattern. when the match expression
    //     // executes, it compates the resulting value of the match expression with the value of each pattern, in 
    //     // sequential order. the first match between the match expression and a matching pattern, returns the
    //     // evaluation of the expressions of the matched pattern to `match` which is the evaluation returned by `match`
    //     pattern => expression,
    //     pattern => expression,
    //     pattern => expression,
    // }

    let m = value_in_cents(Coin::Quarter(State::State1));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);  // how is the `None` enumeration variant converted to a variant that adheres to the  
                                // parameter type `Option<i32>`?
    dbg!(five);      
    
    let dice_roll: u8 = 9;

    match dice_roll {
        5 => { add_fancy_hat() },
        7 => { remove_fancy_hat() },
        // the `other` variable is catching all other possible values of an u8 type to satisfy the exhaustive 
        // requirement of the match expression. the compiller will create a warning if this pattern is not the last
        // pattern in the match expression, since no pattern is reachable after a catch-all pattern
        other => { move_player(other) },
    }

    match dice_roll {
        5 => { add_fancy_hat() },
        7 => { remove_fancy_hat() },
        // the `_` reserved character denotes a catch-all pattern that does not store the value from the match 
        // expression. the compiller will create a warning if we store the match expression return value into a 
        // variable and do not use said variable
        _ => { () }, // an empty tuple signifies an empty expression that returns an empty tuple
    }

    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max);
        _ => (),
    }

    if let Some(max) = config_max {
        // `if let` is syntaxical abstraction that denotes a _non-exhaustive_ conditional expression
        println!("The maximum is configured to be {}", max);
    }

    let coin = Coin::Quarter(State::State1);
    let mut coint = 0;
    
    match coin {
        Coin::Quarter(state) => { println!("State quarter from {:?}", state) },
        _ => { count = count + 1 },
    }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count = count + 1;
    }
}

#[derive(Debug)]
enum State {
    State1,
    State2,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        // the braces are optional if there is only one line of code... (they should be requred regardless, i think?)
        // patterns can have variables to access associated data of enumerations
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => { 5 },
        Coin::Dime => { 10 },
        Coin::Quarter(state) => { 
            // the `state` variable of the `Quarter` pattern is binded to the `State` enumeration variant of the `Coin`
            // variant `Quarter` when `coin` matches this pattern
            dbg!(state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {   // match expressions are _exhaustive_; every possible value must be covered, otherwise the code will not
                // compile. using match expressions with an enumeration like `Option<T>` makes it practical to have  
                // `None` as an enumeration variants that fill the same functionality of null pointers, while still
                // remaining safe to use by default, avoiding classes of errors that have caused billions of united 
                // states dollars in damages

        // patterns behave much like conditional statements (if match..then..=>..evaluate this expression/s)
        Some(i) => { Some(i + 1) }, // since i32 implements the `Copy` trait, i does not take ownership of the i32 value
                                    // within the `Some()` enumeration
        None => { None },   // how is the `None` enumeration variant converted to a variant that adheres to the return 
                            // type `Option<i32>` enumeration?
    }
}

fn add_fancy_hat() { }
fn remove_fancy_hat() { }
fn move_player(num_spaces: u8) { }
fn reroll() { }

// note: it is versatile, practical, and safe to use enumerations for pattern matching within match expressions
// note: `match` and `if let` are used depending on whether the utility should or should not require exhaustibility

// question: are macros statements? (all statements end with a semicolon)
// question: how is control flow evaluated by the compiler?