//! # Introduction
//! Here are the topics we'll cover in this chapter:
//! - How to create threads to run multiple pieces of code at the same time
//! - _Message-parsing_ concurrency, where channels send messages between threads
//! - _Shared-state_ concurrency, where multiple threads have access to some piece of data
//! - The `Sync` and `Send` traits, which extend Rust's concurrency guarantees to user-defined types as well as types
//! provided by the standard library

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;    
    // `mpsc` stands for multiple producers, single consumer
    use std::sync::mpsc;
    
    #[test]
    fn threads_and_channels() {
        // let v = vec![1, 2, 3];

        let (tx, rx) = mpsc::channel();

        let tx1 = tx.clone();

        let handle1 = thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(3));
            }
        });
        
        let handle2 = thread::spawn(move || {
            // the `spawn` function returns a `JoinHandle`, which is an owned value.
            
            // for _i in 1..10 {
                //     println!("here is a vector {:?}", v);
                //     thread::sleep(Duration::from_millis(1));
            // }
            
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {

                // the thread needs to own the `tx` value to be able to send messages through the channel. the `send` method
                // returns a `Result<T, E>` type; if the receiver has already been dropped, the return type will be `Err(E)`
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(2));
            }

            // // Here, we try to print `val` after we've sent it down the channel via `tx.send`. Allowing this would be a bad
            // // idea: once the value has been sent to another thread, that thread could modify or drop it before we try to
            // // use the value again. Potentially, the other thread's modifications could cause errors or unexpected results
            // // due to inconsistent or nonexistent data.
            // println!("val is {}", val);
        });
        
        // for i in 1..5 {
        //     println!("hi number {} from the main thread", i);
        // }
        
        // // the `recv` method will block the main thread's execution and wait until a value is sent down the channel. Once
        // // a value is sent, the `recv` method will return it in a `Result<T, E>` variant. When the transmitter is closes,
        // // `recv` will return an error to signal that the channel is closed. 
        // let received = rx.recv().unwrap();
        // println!("Got: {}", received);
        // // The `try_recv` method doesn't block, but will instead return a `Result<T, E>` immediately: an `Ok` value
        // // holding a message if one is available and an `Err` value if there aren't any messages this time.
        // // Using `try_recv` is useful if this thread has other work to do while waiting for messages. We could write
        // // a loop that calls `try_recv` every so often, handles a message if one is available, and otherwise does other
        // // work for a little while until checking again.
        
        for received in rx {
            // the `for` loop treats `rx` as an iterator and takes care of the calling the `recv` method

            println!("Got: {}", received);
        }

        handle1.join().unwrap();
        handle2.join().unwrap();
    }

    use std::sync::Mutex;
    
    #[test]
    fn mutable_exclusions() {

        // a `Mutex<T>` allows for interior mutability
        let m = Mutex::new(5);
        
        {
            // The call to `lock` returns a smart pointer called `MutexGuard`, wrapped in a `LockResult`.
            let mut num = m.lock().unwrap();
            *num = 6;
        }
        
        println!("m = {:?}", m);
    }
    
    // // the `Arc<T>` type has the same API as the `Rc<T>` type. `Arc<T>` is a less efficient thread safe version of 
    // // the `Rc<T>` type
    // use std::rc::Rc;
    use std::sync::Arc;

    #[test]
    fn mutex_multithreading() {
        // `Rc<T>` does not use any concurrency prerogatives to make sure that changes to the counter can't be interrupted
        // by another thread

        // counter is the equivalent of `Rc::new(RefCell::new(0))` of single threaded programs
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);

            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }

    // note: if another thread owns the lock, this thread will be blocked until the lock becomes available. the call to
    // lock would fail if the previous thread holding the lock panicked.
}

// note: Vec<i32>, where i32 is the cell value.
