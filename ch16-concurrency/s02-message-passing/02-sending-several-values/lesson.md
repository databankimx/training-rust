## Receiving Several Values

A single `recv` takes one value. Often a thread sends a stream of them, and the receiver should handle each as it arrives. The receiver is an iterator, so a `for` loop
takes values until the channel closes:

```rust,ignore
for received in rx {
    println!("Got: {received}");
}
```

The loop ends when every transmitter has been dropped. The spawned thread drops its `tx` when its closure finishes, so the loop stops after the last value.

The sending thread pauses for a second after each message with `thread::sleep`, which makes the arrivals visible. Only one thread sends here, so the order is fixed: each value is
printed as it arrives, one at a time. The output is:

```
Got: hi
Got: from
Got: the
Got: thread
```

Between lines, the program waits, and the pauses come from the sender, not from the receiver. The receiver doesn't do any work between messages, so the wait is entirely the
sender's.

---

### Try it in the runner

Press **Run**. The four lines should appear one at a time, about a second apart.

The comments in `src/main.rs` suggest changing the pause to 200 milliseconds. The lines still arrive in the same order, just faster. Only the timing changes, not the contents.

---

*For more detail, see section 16.2 "Transfer Data Between Threads with Message Passing" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch16-02-message-passing.html).*
