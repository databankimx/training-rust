use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc lets ten threads each own the counter. Mutex lets only one of them change it at a time.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // The guard lives until the closure ends, so each thread holds the lock only while it adds one.
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // Try: change the number of threads from 10 to 100. What does the result print?
}
