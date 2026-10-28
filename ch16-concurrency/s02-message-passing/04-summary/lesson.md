## Summary: Message Passing

Channels let threads communicate by sending values, which avoids sharing memory directly.

* **Sending and receiving:** `mpsc::channel` gives a transmitter and a receiver. `send` moves a value into the channel, and `recv` waits for one. *(See the 16.2 steps.)*
* **Many values:** the receiver is an iterator, which ends once every transmitter is dropped. *(See the receiving several values step.)*
* **Many producers:** cloning the transmitter lets several threads send to one receiver. Their messages can arrive in any order. *(See the multiple producers step.)*
* **One type per channel:** a channel carries a single type. To send different kinds of message, use an enum. *(See the quiz below.)*

Ownership makes these channels safe. Sending a value moves it, so the sender can't use it afterward, and the receiver can't see a value that is still being changed.

---

### Check your understanding

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Consider this program, which sends messages between two threads using an enum:

```rust
use std::{sync::mpsc, thread};

enum ClientMessage { Incr, Get, Quit }
enum ServerMessage { Get(usize) }

fn main() {
    let (server_tx, client_rx) = mpsc::channel();
    let (client_tx, server_rx) = mpsc::channel();
    let server = thread::spawn(move || {
        let mut n = 0;
        loop {
            match server_rx.recv().unwrap() {
                ClientMessage::Quit => break,
                ClientMessage::Incr => n += 1,
                ClientMessage::Get => server_tx.send(ServerMessage::Get(n)).unwrap()
            }
        }
    });

    for msg in [ClientMessage::Incr, ClientMessage::Get, ClientMessage::Quit] {
        client_tx.send(msg).unwrap();
    }

    if let ServerMessage::Get(n) = client_rx.recv().unwrap() {
        println!("{}", n)
    }

    server.join().unwrap();
}
```

What does it print?

○ `0`
◉ `1`
○ `2`
○ The program does not compile
→ Yes. The client sends `Incr`, which makes the server's count 1, then `Get`, which sends back `Get(1)`. The client prints 1, and `Quit` stops the server.
````

````quiz
Consider this program, which sends a `String` and then a number through the same channel:

```rust
use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let s = String::from("Hello world");
        tx.send(s.clone()).unwrap();
        tx.send(s.len()).unwrap();
    });
    let s = rx.recv().unwrap();
    let n = rx.recv().unwrap();
    println!("{s} {n}");
}
```

Does this program compile? If not, on which line does Rust report the problem?

○ This program compiles
○ This program does not compile, on line 7
◉ This program does not compile, on line 8
○ This program does not compile, on line 10
→ Yes. A channel carries one type, decided by the first value sent. `tx.send(s.len())` sends a `usize`, which doesn't match the `String` sent on line 7, so the compiler rejects line 8.
````

---

*The summary is written for this course, and draws on section 16.2 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch16-02-message-passing.html).*
