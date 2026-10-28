use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        // lock blocks until it's our turn, and returns a guard. The guard releases the lock when it goes out of scope.
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    // The lock is released at the end of the block above, so this read doesn't wait.
    println!("m = {}", *m.lock().unwrap());

    // Try: print m with {:?} instead. What does the Debug output show about the data and the lock?
}
