/// Definition of the `Draw` trait
pub trait Draw {
    fn draw(&self);
}

/// Definition of the `Screen` struct with a `components` field holding a vector of trait objects 
/// that implement the `Draw` trait 
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    /// A `run` method on `Screen` that calls the `draw` method on each component
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// /// An alternate implementation of the `Screen` struct using generics and trait bounds
// pub struct HomogeneousScreen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> HomogeneousScreen<T> 
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         /// A `run` method on `Screen` that calls the `draw` method on each component
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

// note: a generic type parameter with trait bounds can only be substiituted with one concrete 
// type object at runtime, whereas trait objects allow for multiple concrete types to fill in for 
// the trait object at runtime. if user collections only have to be homogeneous, using generics and 
// trait bounds is preferable because the definitions will monomorphized at compile time to use the 
// concrete types.