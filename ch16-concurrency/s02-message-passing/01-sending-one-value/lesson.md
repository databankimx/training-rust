## Sending Values with a Channel

Channels are one way for threads to communicate. A channel has a transmitter, `tx`, which sends values, and a receiver, `rx`, which takes them out. The standard library's
`mpsc` module provides them, and `mpsc` stands for *multiple producer, single consumer*. Many threads can hold a transmitter, but only one receiver takes values from the channel.

A common picture is a river. The transmitter puts an item in upstream, and the receiver takes it out downstream. A channel is closed when either end is dropped.

```rust,ignore
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
});

let received = rx.recv().unwrap();
println!("Got: {received}");
```

`mpsc::channel` returns a tuple, and `let (tx, rx)` takes it apart. The spawned thread needs to own `tx`, so the closure is marked `move`. `send` takes the value, and the thread
gives up ownership of it. The method returns a `Result`, which is an error only if the receiver has already been dropped.

`recv` blocks the main thread until a value arrives. It returns `Ok` with the value, or `Err` once every transmitter has been dropped, which tells the receiver that no more values
are coming. The output is:

```
Got: hi
```

The type of the channel isn't written anywhere. Rust infers it from the first value sent, which is a `String` here. A channel can carry only one type, which is the subject of the
last sub-section's quiz.

---

### Try it in the runner

Press **Run**. The output should be `Got: hi`.

The comments in `src/main.rs` suggest calling `recv` a second time. The spawned thread has finished, and its transmitter has been dropped, so `recv` returns an error instead of
waiting forever. Calling `unwrap` on that error stops the program with a panic.

---

*For more detail, see section 16.2 "Transfer Data Between Threads with Message Passing" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch16-02-message-passing.html).*
