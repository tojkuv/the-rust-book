use std::{
	error::Error,
	fmt,
	thread,
	sync::{mpsc, Arc, Mutex},
};

/// `Worker` struct and implementations
struct Worker {
	id: usize,
	thread: thread::JoinHandle<()>,
}

impl Worker {
	fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
		let thread = thread::spawn(move || loop {
			// the `let` statement only keeps in scope the last assigned value to it, unlike
			// `while let`, `if let`, and `match` statements that keep their assignment history in scope;
			// this is an implementation that could change in the future that would allow for the later
			// to only keep in scope their last assigned value
			let job = receiver.lock()
				/* the mutual exclusion primitive ensures that only one thread is requesting a `job()`.
				 the lock is the lifetime of the `MutexGuard` and the lock is unlocked when the guard
				  goes out of scope */
				.expect("the mutual exclusion primitive was poisoned by another thread")
				/* `recv()` blocks the thread if there is no `Job` in the buffer when it makes a request */
				.recv()
				.expect("the sender shutdown");

			println!("Worker {id} got a job; executing.");

			job();
		});

		Worker { id, thread }
	}
}

/// `PoolCreationError` struct and implementations
#[derive(Debug)]
pub struct PoolCreationError;

impl fmt::Display for PoolCreationError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "The number of threads cannot be zero")
	}
}

impl Error for PoolCreationError {}

/// `Job` struct and implementations
type Job = Box<dyn FnOnce() + Send + 'static>;

/// `ThreadPool` struct and implementations
pub struct ThreadPool {
	workers: Vec<Worker>,
	sender: mpsc::Sender<Job>,
}

impl ThreadPool {
	/// Create a new `ThreadPool`.
	///
	/// The size is the number of threads in the pool.
	///
	/// # Panics
	///
	/// The `new` function will panic if the size is zero.
	pub fn new(size: usize) -> ThreadPool {
		assert!(size > 0);

		let (sender, receiver) = mpsc::channel();

		let receiver = Arc::new(Mutex::new(receiver));

		let mut workers = Vec::with_capacity(size);

		// this for loop pushes `size` number of worker threads
		for id in 0..size {
			workers.push(Worker::new(id, Arc::clone(&receiver)));
		}

		ThreadPool { workers, sender }
	}

	pub fn execute<F>(&self, f: F)
	                  where
		                  F: FnOnce() + Send + 'static,
	{
		let job = Box::new(f);

		self.sender.send(job).expect("the receiver shutdown");
	}
}

// note: it's good practice to write unit tests after creating the implementation types
// and empty function signatures to test that they behave like they were intended

// note: `while let`, `if let`, and `match` do not drop temporary values until the end of
// the associated block
