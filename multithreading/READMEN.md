# Using Threads to Run Code Simultaneously

In most current operating systems, an executed program's code is run in a process, and the operating system will manage multiple processes at once. Within a program, you can also have independent parts that run simultaneously. The features that run these independent parts are called *threads*. For example, a web server could have multiple threads so that it could respond to more than one request at the same time.

Threads improve performance, but they also add complexity. Because threads can run simultaneously, there's no inherent guarantee about the order in which parts of your code on different threads will run. This can lead to problems, such as:

- Race conditions, where threads are accessing data or resources in an inconsistent order

- Deadlocks, where two threads are waiting for each other, preventing both threads from continuing

- Bugs that happen only in certain situations and are hard to reproduce and fix reliably

### Creating a New Thread with `spawn`

To createe a new thread, we call the `thread::spawn` function and pass it a closure containing the code we want too run in the new thread. the return type of `thread::spawn` is `JoinHandle`. A `JoinHandle` is an owned value that, when we call the `join` method on it, will wait for its thread to finish.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

### Using `move` Closures with Threads

Using the `move` keyword forces a closure to take ownership of the values it to avoid race conditions.

# Using Message Passing to Transfer Data Between Threads

*Message passing* is where threads or actors communicate by sending each other messages containing data (this is a more predictable way to communicate than sharing a shared state). 

A *channel* is a general programming concept by which data is sent from one thread to another. A channel has two halves: a transmitter and a receiver. One part of your code calls methods on the transmitter with the data you want to send, and another part checks the receiving end for arriving messages. A channel is said to be *closed* if either the transmitter or receiver half is dropped.

We create a new channel using the `mpsc::channel` function; `mpsc` stands for *multiple producer, single consumer*. Rust's standard library implements channels means a channel can have multiple *sending* ends that produce values but only one `receiving` end that consumes those values. The `mpsc::channel` function returns a tuple, the first element of which is the sending end--the transmitter--and the second element is the receiving end--the receiver. The abbreviations `tx` and `rx` are traditionally used in many fields for `transmitter` and `receiver` respectively.

# Shared State

### Using Mutexes to Allow Access to Data from One Thread at a Time

*Mutex* is an abbreviation for *mutual exclusion*, as in, a mutex allows only on thread to access some data at any given time. To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex's *lock*. The lock is a data structure that is part of the mutex that keeps track of whi currently has exclusive access to the data. Therefore, the mutex is described as *guarding* the data it holds via the locking system.

Mutexes have a reputation for being difficult to use because you have to remember tw rules:

- You must attempt to acquire the lock before using the data.

- When you're done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.

### Atomic Reference Counting with Arc<T>

Fortunately, `Arc<T>` *is* a type like `Rc<T>` that is safe to use in concurrent situations. The *a* stands for *atomic*, meaning it's an *atomically reference counted* type. Atomics are an additional kind of concurrency primative that we won;_t cover in detail here: see the standard library documentation for `std::sync::atomic` for more detaills. At this point, you just need to know that atomics work like primitive types but are safe to share across threads.

You might then wonder why all primative types aren'_t atomic and why standard library types aren'_t implemented to use `Arc<T>` by default. The reason is that thread safety comes with a performance penalty that you onlyy want to pay when you really need to. If you're just performing operations on values within a single thread, your code can run faster if it doesn'_t have to enforce the guarantees atomics provide.

 You can divide a calculation into independent parts, split those parts across threads, and then use a `Mutex<T>` to have each thread update the final result with its part.

Note that if you are doing simple numerical operations, there are types simpler than `Mutex<T>` tupes proviided by the `std::sync::atomic modules of the standard library`. These types provide safe, concurrent, atomic access to primative types. We chose to use `Mutex<T>` with a primitive type for this example so we could concentrate on how `Mutex<T>` works.

Another detail to note is that Rust can'_t protect you from all kinds of logic errors when you use `Mutex<T>`. Recall in Chapter 15 that using `Rc<T>` came with the rist of creating reference cycles, where two `Rc<T>` valeus refer to each other, causing memory leaks. Similarly, `Mutex<T>` comes with the rist of creating `deadlocks`. These occur when an operation needs to lock two rersources and two threads have each acquired one of the locks, causing them to wait for each other forever. The standard library API documentation for `Mutex<T>` and `MutexGuard` offers useful information.

# Extensible Concurrency with the Sync and Send Traits

Two concurrency concepts are embedded in the language: the `std:::marker` traits `Sync` and `Send`.

### Allowing Transference of Ownership Between Threads with `send`

The `Send` marker trait indicates that ownership of values of the type implementing `Send` can be transferred between threads. Almost every Rust type implements `Send`, but there are some exceptions, including `Rc<T>`: this type cannot implement `Send` because if you cloned a `Rc<T>` instance and tried to transfer ownership of the clone to another thread, both threads might update the reference count at the same time. For this reason, `Rc<T>`` is implemented for use in single-threaded situations where you don'_t want to pay the thread-safe performance penalty.

Therefore, Rust's type system and trait bounds ensure that you can never accidentally send a `Rc<T>` instance across threads unsafely.

Any type composed entirely of types that implement `Send` is marked as `Send` as well. Almost all promative types implement `Send`, aside from raw pointers.

### Allowing Access from Multiple Threads with `Sync`

The `Sync` marker trait indicates that it is safe for the type implementing `Sync` to be referenced from multiple threads. In other words, any type `T` is `Sync` if `&T` (an immutable reference to `T`) is `Send`, meaning the reference can be sent safely to another thread. Similar to `Send`, primitive types are `Sync`, and types composed entirely of types that are `Sync` are also `Sync`.

The smart pointer `Rc<T>` is also not `Sync` for the same reason that it's not `Send`. The `RefCell<T>`and the family of related `Cell<T>` types are not `Sync`. The implementation of borrow checking that `RefCell<T>` does at runtime is not thread-safe. The smart pointer `Mutex<T>` is `Sync` and can be used to share access with multiple threads.

### Implementing `Send` and `Sync` Manually Is Unsafe

Because types that are made up of `Send` and `Sync` traits are automatically also `Send` and `Sync`, we don'_t have to implement thoose traits manually. As marker traits, they don'_t even have any methods to implement. They're just useful for enforcing invariants related to concurrency.

Manually implementing these traits involves implementing unsafe Rust code. New concurrent types not made up of `Send` and `Sync` parts requires careful thought to uphold the safety guarantees.
