use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // The receiver is an iterator. It yields each value as it arrives, and ends when every transmitter is dropped.
    for received in rx {
        println!("Got: {received}");
    }

    // Try: change the sleep to 200 milliseconds. Does the output still come one line at a time?
}
