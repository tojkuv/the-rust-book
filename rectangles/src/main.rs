// Summary: 
// structs create custom types that provide context. by using structs, associated pieces of data can stay bundled
// together for clearity. `impl` blocks define associated functions (methods, constructors, getters, setters, etc.)

fn main() {
    // using single variables is not clear enough in some cases
    let width1 = 30;
    let height1 = 50;

    let rect1 = (30, 50); // less clear than using single variables

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("The area of the reactangle is {} square pixels.", area(&rect1));
    
    println!("rect1 is {:#?}", rect1); // is this considered passing ownership?
    dbg!(&rect1);
    
    println!("The area of the reactangle is {} square pixels.", rect1.area());
    println!("The rectangle has a nonzero width; it is {}.", rect1.width()); // .width() accesses the field method
    println!("The rectangle has a nonzero width; it is {}.", rect1.width); // .width accesses the field variable

    // the following 2 lines are the same since rust automatically applies referencing and dereferencing
    // p1.distance(&p2);       // C and C++: p1.distance(&p2);
    // (&p1).distance(&p2);    // C and C++: p1->distance(&p2);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2 {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3 {}", rect1.can_hold(&rect3));

    // associated functions are namespaces that can be accessed using the `::` operator
    let rect4 = Rectangle::square(3);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)] // this is an attribute
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
        // all functions defined within an `impl` block are called `associated functions` because they are assocaited
        // with the type named after the `impl` keyword

        // there can be multiple `impl` blocks for the same type (the oposite case would be a boilerplate restriction)
    fn area(self: &Self) -> u32 {
        // `&self` is a short version of `self: &Self`
        // `self` refers to the instance of a structure that the method is being implemented for.
        // `&` is used to pass a reference of the structure instance (intead of passing ownership)
        // passing a `&mut self` reference allows the method to modify the field variables
        // passing `self` ownership is only done when the self is to be transformed into another namespace. in such
        // instance, removing ownership of the original self stops the previous ownner from having access to the old 
        // self after it had been transformed
        self.width * self.height
    }

    fn width(self: &Self) -> bool { // methods can have the same name as field variables
        self.width > 0
    }

    fn height(self: &Self) -> u32 {
        // this method named like the field variable, only returns the value of the field variable; this is a 
        // getter method. field variable can be declared as private with getter methods that provide read only access
        // to the field variable
        self.height
    }

    fn can_hold(self: &Self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        // assciated functions can be defined without a `self` as their first parameter (and thus are not methods).
        // this one is considered a constructor function 
        Rectangle {
            width: size,
            height: size,
        }
    }
}
