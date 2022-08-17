# Closures: Anonymous Functions that Capture Their Environment

Rust's closures are anonymous functions you can save in a variable or pass as arguments to other functions. You can create the closure in one place and then call the closure elsewhere to evaluate it in a different context. Unlike functions, closures can capture values from the scope in which they're defined. We'll demostrate how these closure features allow for code reuse and behavior customization.

### Function traits

1. `FnOnce` applies to closures that can be called at least once. All closures implement at least this trait, because all closures can be called. A closure that moves captured values out of its body will only implement `FnOnce` and none of the other `Fn` traits, because it can only be called once.

2. `FnMut` applies to closures that don't move captured values out of their body, but that *might* mutate the captured values. These closures can be called more than once.

3. `Fn` aplies to closures that don't move capptured values out of their body and that don't mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently. This trait is also important when defining or using functions or types that make use of closures.

# Processing a Series of Items with Iterators

The iterator pattern allows you to perform some task on a sequence of items in turn. An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished. When you use iterators, you don't have to reimplement that logic yourself.

In rust, iterators are *lazy*, meaning they have no effect until you call methods that consume the iterator to use it up.

### The `Iterator` Trait and the `next` method

All iterators implement a trait named `Iterator` that is defined in the standard libary.

Note that iterators binded to variables must be declared mutable: calling the `next` method on an iterator changes iternal state that the iterator uses to keep track of where it is in the sequence. In other words, the `next` method *consumes*, or uses up, the iterator. Each call to `next` consumes an item from the iterator. We don't need to make iterators mutable when they are used by a `for` loop because the loop takes ownership of the iterator and makes it mutable.

Also note that the values we get from the calls to `next` are immutable references to the values in the collections/slices. The `iter` method produces an iterator over immutable references. If we want to create an iterator that takes ownership of values and returns owned values, we can call `into_iter` instead of `iter`. Similarly, if we want to iterate over mutable references, we can call `iter_mut` instead of `iter`.

### Methods that Consume the Iterator

Methods that call `next` are called *consuming adaptors*, because calling them uses up the iterator.

### Methods that Produce Other Iterators

*Iterator adaptors* are methods defined on the `Iterator` trait that don't consume the itertor. Instead, they produce different iterators by changing some aspect of the original iterator.

You can chain multiple calls to iterators to perform complex actions in a readable way. But because all iterators are lazy, you have to call one of the consuming adaptor methods to get results from calls to iterator adaptors.

# Improving Our I/O Project

The functional programming style prefers to minimize the amount of mutable state to make code clearer. Minimizing mutable states also increases our ability to run processes in parallel.

# Comparing Performance: Loop vs. Iterators

Iterators are one of Rust's *zero-cost abstractions*; the abstraction imposes no additional runtime overhead.

*Unrolling* is an optimization that removes the overhead of the loop controlling code and instead generates repetitive code for each iteration of the loop.
