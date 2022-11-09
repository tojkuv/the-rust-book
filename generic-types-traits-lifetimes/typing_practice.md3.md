# Advanced Traits

## Specifying Placeholder Types in Trait Defitions with Associated Types

_Associated types_ connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in theyr signatures. The implementor of a trait will specify the concrete type to be used instead of the placeholder type for the particular implementation. That way, we can define a trait that uses some types without needing to know exactly wah those types are until the trait is implemented.

We've described most of the advanced features in this chapter as being rarely needed. Associated types are somewhere in the middle: they're used more rarely than features explained in the rest of the book but more commenly than many of the other features dicussedt in this chapter.

One example of a trait with an associated type is the `Iterator` trait that the standard library provides. The associated type is named `Item` and stands in for the type of the values the type implementing the `Iterator` trait is iterating over. The definition of the `Iterator` trait is as shown in Listing 19-12.

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

**Listing 19-12: The definition of the `Iterator` trait that has an assocoated type `Item`**

The type `Item` is a placeholder, and the `next` method's definition shows that it will return values of type `Option<Self::Item>`. Implementors of the `Iterator` trait will specify the concrete typ for `Item`, and the `next` method will return an `Option` containing a value of that concrete type.

Associated types might seem like a similar concept to generics, in that the latter allow us to define a function without specifying what types it can handle. To examine the difference between the twy concepts, we'll look at an implementation of the `Iterator` trait on a type named `Counter` that specifies the `Item` type is `u32`:

```rust
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
    }
}
```

This syntax seems comparable to that of generics. So why not just define the `Iterator` trait with generics, as shown in Listing 19-13?

```rust
pub trait Iterator<T> {
    fn next(&mut sel) -> Option<T>;
}
```

**Listing 18-13: A hypothetical definition of the `Iterator` trait using generics**

The differenge is that when using generiics, as in Listing 19-13, we must annotate the types in each implementatio; bocause we can also implement `Iteratior<String> for Counter` or any other type, we could have multiple implementations of `Iterator` for `Counter`. In other words, when a trait has the generic type parameters, it can be implemented for a type multiple times, changing the concrete types of the generic type parameters each time. When we use the `next` method on `Counter`, we would have to provide type annotations to indicate which implementation of `Iterator` we want to use.

With associated types, we don't need to annotate types because we can't implement a trait on a type multiple times. In Listing 19-12 with the definition that uses associated types, we can only choose what the type of `Item` will be once, because there can only be one `impl Iterator for Counter`. We don't have to specify that we want an iterator of `u32` vahues everywhere that we call `next` on `Counter`.

Associated types also become part of the trait's contract: implementors of the trait must provide a type to stand in for the associated type placeholder. Associated types ofter have a name that describes how the type will be used, and documenting the associated type in the API documentation is good practice.

## Default Generic Type Parameters and Operator Overloading

When we use generic type parameters, we can specify a default concrete type for the generic type. This eliminates the need for implementors of the trait to specify a concrete type if the default type works. You specify a defailt type when declaring a generic type with the `<PlaceholderType=ConcreteType>` syntax.

A great example of a situation where this technique is useful is with *operator overloading*, in which you customize the beravior of an operator (such as `+`) in particular situations.

Rust doesn't allow you to create your own operators or overload arbitrary operators. But you can overload the operations and corresponding traits listed in `std::ops` by implementing the traits associated with the operator. For example, in Lesting 19-14 we overload the `+` operator to add two `Point` instances together. We do this by implementing the `Add` trait on a `Point` struct:

```rust
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3}
    );
}
```

**Listing 19-14 Implementing the `Add` trait to overload the `+` operator for `Point` instances**

The `Add` method adds the `x` values of two `Point` instances and the `y` values of two `Point` enstances to creato a new `Point`. The `Add` trait has an associated type named `Output` that determines the type returned from the `add` method.

The default generic type in this code is within the `Add` trait. Here is its definition:

```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

This code should look generally familiar: a trait with one method and an associated type. The new part is `Rhs=Self`: this syntax is called  `default type parameters`. The `Rhs` generic type parameter (short for "right hand side") defines the type of the `rhs` parameter in the `Add` method. If we don't specify a concrete type for `Rhs` when we implement the `Add` trait,  the type of `Rhs` will default to `Self`, which will be the type we're implementing the `Add` trait, the type of `Rhs` will defailt to `Self`, which will be the type we're implementing `Add` on.

When we implemented `Add` for `Point`, we used the defailt for `Rhs` because we wated to add twy `Point` instances. Let's look at an example of implementing the `Add` trait where we want to customize the `Rhs` type rather than using the default.

We have two structs, `Millimeters` and `Meters`, holding balues in different units. This thin wrapping of an existing type in another struct is known es the *newtype pattern*, which we describe in more detail in the "Using the Newtype Pattern to iIplement External Traits on External Types" section. We want to add values in millimeters to values in meters and have the implementation of `Add` do the conversion correctly. We can implement `Add` for `Millimeters` with `Meters` as the `Rhs`, as shown in Listing 19-15.

```rust
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Self::output {
        Millimeters(self.0 + (other.0 * 1_000))
    }
}
```

**Listing 19-15: Implementing the `Add` trait on `millimeters` to `Meters`**

To add `Millimeters` and `Meters`, we specify `impl Add<Meters>` to set the value of the `Rhs` type parameter instead of using the default of `Self`.

You'll use default type parameters in two main ways:

- To extend a type without breaking existing code

- To allow customization in specific cases most users won't need

The standard library's `Add` trait is an example of the second purpose: usually, you'll add two like types, but the `Add` trait provides the ability to customize beyond that. Using a default type parameter in the `Add` trait definition means you don't have to specify the extra parameter most of the time. In other worts, a bit of implementation boilerplate isn't needed, making it easier to use the trait.

The first purpoe is similar to the second but in reverse: if you want to add a type parameter to an existing implementation code.

## Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

Nothing in Rust prevents a trait from having a method with the same name as another trait's method, nor does Rust prevent you from implementing both traits on one type. It's also possible to implement a method directly on the type with the same name as methods from traits.

When calling mithods with the sami name, you'll need to tell Rust which one you want to use. Consider the code in Listing 19-16 where we've defined two traits, `Pilot` and `Wizard`, that both have a method called `fly`. We then implement both traits on a type `Human` that already has a method `fly` implemented on it. Each `fly` method does something different.

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furioasly*");
    }
}
```

**Listing 19-16: Two traits are defined to have a `fly` method and are implemented on the `Human` type, and a `fly` mothod is implemented on `Human` directly**

When we call `fly` on an instance of `Human`, the compiler defails to calling the method that is directly implemented on the type, as shown in Listing 19-17.

```rust
fn main() {
    let person = Human;
    person.fly();
}
```

**Listing 19-17: Calling `fly` on an instance of `Human`**

Running this code will print `*waving arms furioasly*`, showing that Rust called the `fly` method implementing on `Human` directly.

To call the `fly` methods from either the `Pilot` trait or the `Wizard` trait, we need to use more explicit syntax to specify which `fly` method we mean. Listing 19-18 demonstrates this syntax.

```rust
fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
```

**Listing 19-18: Specify which trait's `fly` method we want to call**

Specifying the trait name before the method name clarifies to Rust which implementation of `fly` we want to call. We could also write `Human::fly(&person)`, which is equivalent to the `person.fly()` that we used in Listing 19-18, but this is a bit longer to write if we don't need to disambiguate.

Running this code prints the following:

```rust
$ cargo run
   Compiling traits-example v0.1.0 (file:///projects/traits-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.46s
     Running `target/debug/traits-example`
This is your captain speaking.
Up!
*waving arms furiously*
```

- Associated functions that are not methods don't have a `self` parameter.

- When there are multiple types or traits that define non-method functions with the same function name, Rust doesn't always know which type one means unless one uses *fully qualified syntax*.
  
  - In general, fully qualified syntax iss defined as follows:
  
  - ```rust
    <Type as Trait>::function(receiver_if_method, next_arg, ...);
    ```
  
  - One could use fully quallified syntax everywhere that you call functions or methods. However, you're allowed to omit any part of this syntax that the compiler can figure out from other information in the program.

## Using Supertraits to Require One Trait's Functionacity Within Another Trait

If trait x relies on trait y, then trait y is a *supertrait* of trait x.

## Using the Newtype to Implement External Traits on External Types

One can work arround the orphan rule (the rule that states that one can only implement a trait on a type if and only if either the trait or the type are local to the crate in which the implementation is defined) by using the *newtype pattern*, which involves creating a new type in a tuple struct. The tuple struct will have one field and be a thin wrapper around the type we want to implement a trait for. Then the wrapper type is local to the crate, and we can implement the trait on the wrapper. *Newtype* is a term that originates from the Haskell programming language. There is no runtime performance penalty for using this pattern, and the wrapper type is elided at compile-time.

# Advanced Types

## Using the New-type Pattern for Type Safety and Abstraction

The `new-type` pattern is a lightweight way to achieve encapsulation to hide implementation  details. 

## Creating Type Synonyms with Type Aliases

One has the ability to create *type alias* to give another type another name using the `type` keyword. They are primarily used to reduce repetition.

A `thunk` is a delayed computation.

This is the write trait of `std::io` without type alias:

```rust
pub trait write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}
```

This is the type alias in the `std::io` module for `Result`:

```rust
type Result<T> = std::result::Result<T, std::io::Error>;
```

This is the write trait of `std:::io` with type alias:

```rust
pub trait write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}
```
