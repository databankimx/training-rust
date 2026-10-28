## Sending Data Between Tasks with an Async Channel

Chapter 16 sent values between threads with a channel. Async has a channel with the same shape: `trpl::channel` returns a sender and a receiver, and the receiver is used with
`await`. Two differences matter.

The first is that `recv` returns a future, rather than blocking. The receiver hands control back to the runtime while it waits, so other futures can run. The `send` call doesn't block
either, because this channel has no size limit.

The second is the end of the channel. `recv` resolves to `Some(value)` for each message, and to `None` once the sending end has been dropped. A `while let` loop is the natural way to
read a stream of values from it:

```rust,ignore
while let Some(value) = rx.recv().await {
    println!("received '{value}'");
}
```

The sender runs in its own async block, and it has to own `tx`. That's why the block is `async move`, as Chapter 16 described for threads. When the block finishes, `tx` is dropped, the
channel closes, and the receiver's loop ends. Without `move`, `tx` would stay alive until the outer block ended, and the receiver would wait forever.

The two futures are run together with `trpl::join`, the same way as the last step. The output is:

```
received 'hi'
received 'from'
received 'the'
received 'thread'
```

The messages arrive half a second apart, and the program ends as soon as the last one has been received.

---

### Try it in the runner

Press **Run**. The output should match the four lines above, with a half-second gap between them.

The comments in `src/main.rs` suggest changing the delay to one second. The output is the same, and the run takes about twice as long.

---

*For more detail, see section 17.2 "Applying Concurrency with Async" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch17-02-concurrency-with-async.html).*
