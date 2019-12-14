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
/// This function may create more threads than there are CPU cores, because
/// it's meant to be used for IO-heavy operations or other tasks that have a
/// similarly high wait time.
///
/// # Arguments
/// `num_threads` - The number of threads to spawn
/// `queue`       - The queue to process
/// `on_entry`    - Called for each entry in the queue
///                 Argument is the entry
/// `on_retry`    - Called before a worker thread retries because the queue is empty
///                 Argument is the number of retries that have been attempted
///
/// # Example
/// ```rust
/// let queue = SegQueue::new();
/// queue.push(7);
/// queue.push(42);
///
/// // This call will print `7` and `42` in any order
/// process_queue(
///     4,
///     &queue,
///     |num| println!("{}", num),
///     |tries| print!()
/// );
/// ```
pub fn process_queue<F1, F2, T>(num_threads: usize, queue: &SegQueue<T>, on_entry: F1, on_retry: F2)
where
    F1: Sync + Fn(T),
    F2: Sync + Fn(usize),
    T: Send,
{
    // No use in spawning all these threads and processing an empty queue
    if queue.len() == 0 {
        return;
    }

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
    })
    .expect("Threading error"); // TODO add better error handling
}

#[cfg(test)]
mod test {
    use crossbeam::queue::SegQueue;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::process_queue;

    #[test]
    fn count_items() {
        let queue: SegQueue<usize> = SegQueue::new();
        for i in 1..20 {
            queue.push(i);
        }

        let total = AtomicUsize::new(0);

        process_queue(
            4,
            &queue,
            |i| {
                total.fetch_add(i, Ordering::SeqCst);
            },
            |_| (),
        );

        let total = total.into_inner();

        // We don't need to test the inner workings of `process_queue()`
        // since the heavy lifting is handled by the `crossbeam` crate.
        // So the only thing we should check is if the entire queue has
        // been processed.
        assert_eq!(total, 190);
    }
}
