# Unsafe Rust

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

Unsafe Rust has two new types called *raw pointers* that are similar to references. as with references, raw pointers can be immutable or mutable and are written as `*const T` and `*mut T`, respectively. The asterisk isn't the dereference operator; it's part of the type name.*immutable* means that the pointer can't be directly assigned to after being dereferenced.

Different from references and smart pointers, raw pointers:

- Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location

- Aren't guaranteed to poinnt to valid memory

- Are allowed to be null

- Don't implement any automatic cleanup

By opting out of having Rust enforce these guarantees, you can give up guaranteed safety in exchange for greater performance or the ability to interface with another language or hardware where Rust's guarantees don't apply.


