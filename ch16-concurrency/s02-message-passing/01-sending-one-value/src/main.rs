use std::sync::mpsc;
use std::thread;

fn main() {
    // mpsc::channel returns a transmitter, tx, and a receiver, rx. The type is inferred from what is sent.
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // recv blocks until a value arrives, then returns it in a Result.
    let received = rx.recv().unwrap();
    println!("Got: {received}");

    // Try: call rx.recv() a second time. What happens once tx has been dropped?
}
