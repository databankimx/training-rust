## Multiple Producers

`mpsc` means *multiple producer, single consumer*, and this step uses the multiple-producer part. Cloning a transmitter gives a second handle that sends into the same receiver.
Each thread gets its own transmitter, and the receiver takes values from all of them:

```rust,ignore
let tx1 = tx.clone();
thread::spawn(move || {
    // sends hi, from, the, thread
});

thread::spawn(move || {
    // sends more, messages, for, you
});
```

The first thread gets the clone, and the second gets the original. Each closure is `move`, so it owns its transmitter. Nothing is shared between the threads, except the receiver.

The two threads run at the same time, so their messages can reach the receiver in any order. Running the book's version several times shows that. This sample collects every message into a
vector, and sorts it before printing:

```rust,ignore
let mut received: Vec<String> = rx.iter().collect();
received.sort();
```

`rx.iter()` keeps going until every transmitter has been dropped, so `collect` waits for both threads to finish. Sorting then puts the messages in a fixed order. The output is:

```
Got: for
Got: from
Got: hi
Got: messages
Got: more
Got: the
Got: thread
Got: you
```

Sorting is one way to get fixed output from concurrent code. A real program would usually process each message as it arrives, and keep the order where it matters, such as with
a sequence number in each message.

---

### Try it in the runner

Press **Run**. The output should match the eight lines above.

The comments in `src/main.rs` suggest removing the `sort` call. Then the order follows the arrival of the messages, and it can change from one run to the next.

---

*For more detail, see section 16.2 "Transfer Data Between Threads with Message Passing" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch16-02-message-passing.html).*
