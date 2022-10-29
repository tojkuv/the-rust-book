``# Unsafe Rust

To switch to unsafe Rust, use the `unsafe` keyword and then start a new block that holds the unsafe code.

### Unsafe Superpowers

- Dereference a raw pointer

- Call an unsafe function or method

- Access or modify a mutable static variable

- Implement an unsafe trait

- Access fields of `union`s

The `unsafe` keyword does not turn off the borrow checker or disable any other of Rust's safety checks: if you use a reference in unsafe code, it will still be checked.

Keep `unsafe` blocks small; you'll be thankful later when you investigate memory bugs.

Its best to enclose unsafe code within a safe abstraction and provide a safe API.

### Dereferencing a Raw Pointer

Unsafe Rust has two new types called *raw pointers* that are similar to references. as with references, raw pointers can be immutable or mutable and are written as `*const T` and `*mut T`, respectively. The asterisk isn't the dereference operator; it's part of the type name. *Immutable* means that the pointer can't be directly assigned to after being dereferenced.

Different from references and smart pointers, raw pointers:

- Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location

- Aren't guaranteed to point to valid memory

- Are allowed to be null

- Don't implement any automatic cleanup

By opting out of having Rust enforce these guarantees, you can give up guaranteed safety in exchange for greater performance or the ability to interface with another language or hardware where Rust's guarantees don't apply.

Raw pointers can be created in safe rust, but they must be dereferenced within `unsafe` blocks.

### Calling an Unsafe Function or Method
Bodies of unsafe functions are effectively `unsafe` blocks, so to perform other unsafe operations within an unsafe function, we don't need to add another `unsafe` block.
### Creating a Safe Abstraction over Unsafe Code
Safe functions that use unsafe code are a safe abstraction for unsafe code, the function is responsible for a safe implementation of unsafe code.
### Using `extern` Functions to Call External Code
An FFI is a way for a programming language to define functions and enable a different (foreign) programming language to call those functions.
### Accessing or Modifying a Mutable Static Variable
Static variables can only store references with the `'static` lifetime. Static variables can be immutable and mutable. Accessing and modifying mutable static variables is unsafe--two threads accessing and modifying the same mutable static variable can create a data race. it's preferable to use concurrency techniques and thread-safe smart pointers.

A difference between constants and immutable static variables is that values in a static variable have a fixed address in memory--using the value will always access the same data.
