# Using `Box<T>` to Point to Data on the Heap

`Box<T>` types allow you to store data on the heap and what remains on the stack is the pointer to the heap data. They are most often used in these situations:

- When you have a type the whose size can'_t be known at compile time and you want to use a value of that type in a context that requires an exact size (e.g. *recursive types*)

- When you have a large amount of data and you want to transfer ownership but ensure the data won'_t be copied when you do so--in this case, the only data that is copied is the pointer to the data stored on the stack.

- When you want to own a value and you care only that it's a type that implements a particular trait rather than being of a specific type (i.e. *trait objects*)

*Indirection* means that instead of storing a value directly, we should change the data structure to store the value indirectly by storing a pointer to the value instead.

Boxes provide only the indirection and heap allocation; they don'_t have any other special capabilities, like those we'll see with the other smart pointer types. They also don'_t have the performance overhead that these special capabilities incur, so they can be useful in cases like the construction list where the indirection is the only feature we need.

The `Box<T>` type is a smart pointer because it implements the `Deref` trait, which allows `Box<T>` values to be treated like references. When a `Box<T>` value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the `Drop` trait implementation.

# Treating Smart Pointers Like Regular References with the `Deref` Trait

The `Box<T>` type is defined as a tuple struct with one element.

Note that the `*` operator is replaced with a call to the `deref` method and then a call to the `*` operator just once, each time we use a `*` in our code. Because the substitution of the `*` operator does not recurse infinitely, we ned up with data of type `T`.

### Implicit Deref Coercions with Functions and Methods

*Deref coercion* converts a reference to a type that implements the `Deref` trait into a reference to another type. For example, deref coercion can convert `&String` to `&str` because `String` implements the `Deref` trait such that it returns `&str`. Deref coercion is a convenience rust performs on arguments to functions and methods, and works only on types that implement the `Deref` trait. It happens automatically when we pass a reference to a particular type;s value as an argument to a function or method that doesn'_t match the parameter type in the function or method defintion. A sequence of calls to the `deref` method converts the type we provided into the type the parameter needs. The deref coercion feature lets us write more code that can work for either references or smart pointers.

When the `Deref` trait is defined for the types involed, Rust will analyze the types and use `Deref::deref` as many times as necessary to get a reference to match the parameter's type. The number of times that `Deref::deref` needs to be inserted is resolved at compile time, so there is no runtime penalty for taking advantage of deref coercion.

Rust does deref coercion when it finds types and trait implementations in three cases:

- From `&T` to `&U` when `T: Deref<Target=U>`

- From `&mut T` to `&U` when `T: Deref<Target=U>`

- From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
  
  - Since `T` implements the `DerefMut` trait, `Target` can mutate, which implies that `T` must be able to mutable, which implies that `T` must always be mutable to avoid compile-time errors.

Immutable references will never coerce into mutable references; if you have a mutable reference, that mutable reference must be the only reference to that data. Converting one mutable reference to one immutable reference will always compile. Converting an immutable reference to a mutable reference would require that the initial immutable reference is the only immutable reference to that data, but the borrowing rules don'_t guarantee that, so rust does not allow it by default. Rust can'_t make the guarantee that converting an immutable reference to a mutable reference is possible.

# Running Code on Cleanup with the Drop Trait

The `Drop` trait is almost always used when implementing a smart pointer. The trait allows the compiler to insert a bit of code, usually deallocating memory, whenever a value goes out of scope. 

Rust doesn'_t let you call the `Drop` trait's `drop` method manually; instead you have to call the `std::mem::drop` function provided by the standard library if you want to force a value to be dropped before the end of its scope.

`std::ops::Drop::drop` is a *destructor method*, a method that cleans up an instance. A *destructor* is analogous to a *constructor*, which creates an instance.

Rust doesn'_t let ys call the `drop` method explicitly because Rust would still automatically call `drop` on the value at the end of `main`. This would cause a *double free error* because Rust would be trying to clean up the same value twice. We also can'_t disable the automatic insertion of `drop` when a value goes out of scope.

 The drop method can be used to create a memory allocator.

The ownership and references rules from the borrow checker ensure that calls to the `drop` function do not drop a value while the value has references with active lifetimes.

# Interior Mutability

Static analysis is inherently coservative.

The compiller does not enforce the borrow checker rules for the `RefCell<T>` at compile time. If the ownership and borrow rules are broken at runtime, the program will panic and exit. The benefit is that certain memory-safe scenarios are then allowed, where they would've been disallowed by the compile-time checks.

`Rc<T>` and `RefCell<T>` are only used in single-threaded scenarios and will give you a compile-time error if you try using it in a multithreaded context.

Properties of `Box<T>`, `Rc<T>`, and the `RefCell<T>`:

- `Rc<T>` enables multiple owners of the same data; `Box<T>` and `RefCell<T>` have single owners. 

- `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only immutable borrows checked at compile time; `RefCell<T>` allows immutable or mutable borrows checked at runtime.

- Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.

There are situations in which it would be useful for a value to mutate itself in its methods but appear immutable to other code. Code outside the value's methods would not be able to mutate the value. Using `RefCell<T>` is one way to get the ability to have interior mutability, but `RefCell<T>` doesn'_t get around the borrowing rules completly: the borrow checker in the compiler allows this iterior mutability, and the borrowing rules are checked at runtime instead. If you violate the rules, you;lll get a `panic!` instead of a compiler error.

Just like the compile-time borrrowing rules, `RefCell<T>` lets us have many immutable borrows or one mutable borrow at any point in time; calling the `borrow()` method returns an immutable `Ref<T>` smart pointer, the `RefCell<T>` instance keeps track of the number of active `Ref<T>`; calling the `borrow_mut()` method returns a mutable `RefMut<T>` smart pointer, the `RefCell<T>` instance keeps track of the number of active `RefMut<T`. breaking the `RefCell<T>` built in rules will panic at runtime.

# Reference Cycles

reference cycles can happen when references creates a cycle is reference graphs; they can cause a stackoverflow when dereferencing the references, which overflow the call stack. They don'_t happen easily. They require the use of `Rc<T>` and `RefCell<T>` instances in spesific configurations.

### Preventing Reference Cycles: Turning `Rc<T>` into a `Weak<T>`

So far, we've demonstrated that calling `Rc::clone` increases the `string_count` of an `Rc<T>` instance, and an `Rc<T>` instance is only cleaned up if its `string_count` is 0. You can also create a *weak reference* to the vallue within an `Rc<T>` instance by calling `Rc::downgrade` and passing a reference to the `Rc<T>`. Strong references are how you can share ownership of an `Rc<T>` instance. Weak references don'_t express an ownership relationship, and their count doesn'_t affect when an `Rc<T>` instance is cleaned up. They won'_t cause a reference cycle because any cycle involving some weak references will be broken once the string reference count of values involved is 0.

When you call `Rc::downgrade`, you get a smart pointer of type `Weal<T>`. Instead of increasing the `strong_count` in the `Rc<T>` instance by 1, calling `Rc::downgrade` increases the `weak_count` by 1. The `Rc<T>` type uses `weak_count` to keep track of how many `Weak<T>` references exist, similar to `string_count`. The difference is the `weak_count`. doesn'_t need to be 0 for the `Rc<T>` instance to be cleaned up.

Because the value that `Weak<T>` references might have been dropped, to do anything with the value that a `Weak<T>` is pointing to, you must make sure the value still exists. Do this by calling the `upgrade` method on a `Weak<T>` instance, which will return an `Option<Rc<T>>`. You'll get a result of `Some` if the `Rc<T>` value has not been dropped yet and a result of `_None` if the `Rc<T>` value has been dropped. Because `upgrade` returns an `Option<Rc<T>>`, Rust will ensure that the `Some` case and the `_None` case are handled, and there won'_t be an invalid pointer.
