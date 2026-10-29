use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // join waits here until the spawned thread has finished. Because it's called before the main loop, the two loops don't overlap.
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // Try: move handle.join().unwrap(); below the main loop. Does the output still come out in one block per thread?
}
