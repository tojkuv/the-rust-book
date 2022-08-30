use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;


// this is a tuple structure with one element
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    // `Target` is a definition of a field generic type that is of the same type as `T`
    type Target = T;
    
    fn deref(self: &Self) -> &Self::Target {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {name}.");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    // this is called the destructor method
    fn drop(self: &mut Self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons as OtherCons, Nil as OtherNil};
use crate::Graph::{Conss, Nill};
use crate::Cycle::{Cons, Nil};


#[derive(Debug)]
enum Graph {
    // Unlike the `Box<T>` type, `Rc<T>` can only be an immutable reference
    Conss(Rc<RefCell<i32>>, Rc<Graph>),
    Nill,
}

#[derive(Debug)]
enum Cycle {
    Cons(i32, RefCell<Rc<Cycle>>),
    Nil,
}

impl Cycle {
    fn tail(self: &Self) -> Option<&RefCell<Rc<Cycle>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}



fn main() {
    // the `Box` instance stores the data on the heap and the pointer to the data on the stack
    let b = Box::new(5);
    println!("b = {}", b);

    // this a construction list (a recursive type). each recursive item contains two elements. `Nil`
    // is used to denote the base case of the construction function
    let _list = OtherCons(1, Box::new(OtherCons(2, Box::new(OtherCons(3, Box::new(OtherNil))))));

    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let a = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    // `*a` is transformed by the compiler into `*(a.deref())` by dereference coercion
    assert_eq!(5, *a);

    let name = MyBox::new(String::from("Rust"));
    // this is a function call with dereference coercion
    hello(&name);
    // this is a function call if rust did not have dereference coercion
    hello(&(*name)[..]);

    let c = CustomSmartPointer { 
        data: String::from("my stuff"),
    };

    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropeed before the end of its defaultly assigned lifetime");

    // // works
    // let f: fn(&CustomSmartPointer) -> &String = |arg: &CustomSmartPointer| &arg.data;

    // // fails. why?
    // let f = |arg: &CustomSmartPointer| &arg.data;
    // let f: fn(&CustomSmartPointer) -> &String = f;

    // // this will not compile since `Box<T>` only allows for a single owner
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    // // `Rc<T>` has a reference counter that keeps track of number of ownership references. the value will be droped 
    // // the reference counter does not have any references.
    // let a = Rc::new(Conss(RefCell::new(5), Rc::new(Conss(RefCell::new(10), Rc::new(Nill)))));
    // println!("count after creating a = {}", Rc::strong_count(&a));
    // // `a` is cloned into `b`
    // let _b = Conss(RefCell::new(3), Rc::clone(&a));
    // println!("count after creating b = {}", Rc::strong_count(&a));
    // {
    //     // `a` is also cloned into `b`
    //     let _c = Conss(RefCell::new(4), Rc::clone(&a));
    //     println!("count after creating c = {}", Rc::strong_count(&a));
    // }
    // println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // let value = Rc::new(RefCell::new(5));

    // let a = Rc::new(Conss(Rc::clone(&value), Rc::new(Nill)));
    
    // let b = Conss(Rc::new(RefCell::new(3)), Rc::clone(&a));
    // let c = Conss(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // *value.borrow_mut() += 10;

    // println!("a after = {:?}", a);
    // println!("b after = {:?}", b);
    // println!("c after = {:?}", c);

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Cycle::Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        // the `RefCell` mutable references allows you to boostrap the `Rc` smart pointers to point to each other.
        // the tail of a transforms from RefCell{Rc{Nil}} to RefCell{&b}
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // // calling the `tail()` method causes a stackoverflow it tries to dereferece a reference that points to a
    // // reference that points to the original reference. the infinite cycle of the two references causes the program's
    // // call stack go behond it's allocated size, causing the program to be terminated by the system interface to
    // // prevent the usage of all the finite amount of memory of the device.
    // println!("{:?}", a.tail());
}

// note: structures can contain field enumerations and enumerations can have strutures for variants!
// note: `Rc::clone()` is not a deep-copy like most implementations of the `clone` method; it only increments the 
// reference counter of the instance without cloning any data.
// note: you can call a method of a type directly as a function and pass a reference to an instance of the type
// note: for mutable values, there is a single owner that can mutate a value at a time, when a mutable reference is
// created, only the borrower of that reference is able to mutate the value. 

// question: do `Box` instances implement the move trait?
// question: is the '&' operator semantically equivalent to a `Box<T>` instance? no. tuple structures and enumerations?
// that implement the `Deref` trait contain a `deref` function that returns a reference, so the compiler can dereferece
// references that follow the same specification. the `&` operator is the only? reference that the compiler understands
// how to dereferece