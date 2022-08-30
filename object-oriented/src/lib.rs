mod gui;
mod application;
mod posts;

#[cfg(test)]
mod tests {
    use crate::gui::Screen;
    use crate::application::{ SelectBox, Button };

    #[test]
    fn trait_objects() {
    /// using trait objects to store values of different types that implement the same trait.
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes!"),
                        String::from("Maybe"),
                        String::from("No"),
                    ],
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("OK"),
                }),
                // Box::new(String::from("compile-time error")),
            ],
        };

        screen.run();
    }

    use crate::posts::Post;

    #[test]
    fn posts_test() {
        let mut post = Post::new();

        post.add_text("this is the content");

        let post = post.request_review();

        let post = post.approve();

        println!("{}", post.content());
    }
}

// note: static dispatch is when the compiler knows what method you're calling at compile time.

// note: dynamic dispatch is when the compiler can't tell at compile time which method you're calling. in dynamic 
// dispatch cases, the compiler emits code that at runtime will figure out which method to call. Rust uses the pointers 
// inside the trait object to know which method to call. This lookup incurs a runtime cost that doesn't occur with 
// static dispatch. Dynamic dispatch also prevents the compiler from choosing to inline a method's code, which in turn 
// prevents some optimizations.