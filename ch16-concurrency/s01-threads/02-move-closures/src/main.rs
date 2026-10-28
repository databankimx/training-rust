use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // move gives the closure ownership of v, so the spawned thread can use it even if this function ends first.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();

    // Try: remove the move keyword. What does the compiler say, and why?
}
