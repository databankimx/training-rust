use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    // Cloning the transmitter gives a second producer. Both send into the same receiver.
    let tx1 = tx.clone();
    thread::spawn(move || {
        for val in ["hi", "from", "the", "thread"] {
            tx1.send(String::from(val)).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        for val in ["more", "messages", "for", "you"] {
            tx.send(String::from(val)).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // The messages from the two threads can arrive in any order. Collecting and sorting them gives the same output every run.
    let mut received: Vec<String> = rx.iter().collect();
    received.sort();
    for message in received {
        println!("Got: {message}");
    }

    // Try: remove the sort call. Run it several times. Does the order change?
}
