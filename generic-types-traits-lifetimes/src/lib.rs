mod traits {
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

    impl Dog {
        fn baby_name() -> String {
            String::from("Sofie")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::*;

    #[test]
    fn methods_with_the_same_name() {
        let person = Human;
        // person.fly();

        let person = Human;
        Pilot::fly(&person);
        Wizard::fly(&person);
        // person.fly();

        eprintln!("A baby dog is called a {}", Dog::baby_name());

        Dog::baby_name();
    }
}

// question: if one imports an external struct that implements multiple methods with the same method signature, why must one
// specify the scope of the method implemented directly into the object, unlike an object that was declared locally?
