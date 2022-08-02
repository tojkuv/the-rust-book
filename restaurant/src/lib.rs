mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // this field is private
    }

    impl Breakfast {
        // since the `Breakfast` structure has a private field, the structure must implement a constructor function
        // since the `seasonal_fruit` field variable cannot be accessed outside the module, only associated functions
        // can access the private fields. 
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mmind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // the next line won't compile if we uncomment it; we're not allowed to see or modify the seasonal fruit that
    // comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

mod front_of_house {
    pub mod hosting {   // this module is public; it can be brought into any scope that is within the super module of
                        // `front_of_house` since `front_of_house` is a private module
        pub fn add_to_waitlist() {} // this function is public; it can be used by any library or binary that brings it
                                    // into scope
    }
}

use crate::front_of_house::hosting; // `use` brings into scope the public `hosting` module using the absolute path


mod customer {
    pub fn eat_at_restaurant1() {
        super::hosting::add_to_waitlist(); // this won't compile since `hosting` is not within the scope of `customer` 
                                    // the `eat_at_restaurant` function calls the `add_to_waitlist` function
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant2() {
    add_to_waitlist();  // it is not clear where the `add_to_waitlist` function is declared--due to this, it is good
                        // coding practice to only bring into scope the module of the function and not the function
                        // itself 
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// use crate::front_of_hosue::hosting; // this module is only available to the global scope as private
pub use crate::front_of_house::hosting;     // this module is available to functions, variables, et cetera in the
                                            // global scope. `hosting` is defined in the `front_of_house` module
                                            // but since it is brought into the scope of the `restaurant` module as
                                            // public, it can be brought into the scope of other modules using the 
                                            // path of the `restaurant` module, as opposed to the direct path of
                                            // the hosting module.

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}