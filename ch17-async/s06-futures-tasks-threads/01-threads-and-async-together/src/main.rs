use std::{thread, time::Duration};

fn main() {
    let (tx, mut rx) = trpl::channel();

    // Blocking work runs on a thread, because it would stop an async task from making progress.
    thread::spawn(move || {
        for i in 1..11 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // The receiving side is async, so it can wait for each message without blocking the rest of the program.
    trpl::block_on(async {
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
    });

    // Try: change 11 to 5 in the range. How long does the program take now?
}
