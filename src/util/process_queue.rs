use std::thread;
use std::time::Duration;

use crossbeam::queue::SegQueue;
use crossbeam::scope;

/// Since the queue may be empty at one point but new paths could get added
/// by another thread right after, every thread should try the queue several
/// times before terminating.
const MAX_TRIES: usize = 5;

/// Time a thread should wait in milliseconds before each retry, after it
/// receives an empty queue
const TIMEOUT_MS_BETWEEN_TRIES: u64 = 50;

/// Process a queue on multiple threads
///
/// This function creates more threads than there are CPU cores, because
/// it's meant to be used for IO-heavy operations or other tasks that
/// have a similarly high wait time. When running functions that use
/// a lot of CPU power, the threads will get in each other's way and
/// performance will be less than optimal.
///
/// # Arguments
/// `queue`    - The queue to process
/// `on_entry` - Called for each entry in the queue
///              Argument is the entry
/// `on_retry` - Called before a worker thread retries because the queue is empty
///              Argument is the number of retries that have been attempted
///
/// # Example
/// ```rust
/// let queue = SegQueue::new();
/// queue.push(7);
/// queue.push(42);
///
/// // Below will print `7` and `42` in any order
/// process_queue(
///     &queue,
///     |num| println!("{}", num),
///     |tries| print!()
/// );
/// ```
pub fn process_queue<F1, F2, T>(queue: &SegQueue<T>, on_entry: F1, on_retry: F2)
	where F1: Sync + Fn(T), F2: Sync + Fn(usize), T: Send {

	// I've set the number of threads to spawn to four times the CPU cores
	// because at this point the balance between read speed and CPU usage
	// seemed to be most ideal in my very limited tests with a quad-core
	// (8 threads) CPU on an SSD. My HDD caps out at 100% read speed with
	// just a few threads, so it doesn't make much of a difference there.
	// Real-world tests and experience may give different results and the
	// number of threads may need to be adjusted later on.
	let num_threads = num_cpus::get() * 4;

	// Crossbeam scoped threads
	scope(|s| {
		for _ in 0..num_threads {
			s.spawn(|_| {
				let mut tries = 0;

				while tries < MAX_TRIES {
					// If there's an entry in the queue, handle it
					if let Ok(entry) = queue.pop() {
						tries = 0;

						on_entry(entry);
						continue;
					}

					// If no entry was found, sleep for a little while so
					// the other threads get a chance to put something new
					// in the queue
					tries += 1;
					on_retry(tries);

					thread::sleep(Duration::from_millis(TIMEOUT_MS_BETWEEN_TRIES));
				}
			});
		}
	}).expect("Threading error"); // TODO add better error handling
}
