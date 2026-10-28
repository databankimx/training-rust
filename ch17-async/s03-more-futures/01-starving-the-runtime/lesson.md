## Starving the Runtime

A runtime can switch between futures only at an `await` point. Everything between two await points runs without interruption. If one future does a lot of work without awaiting,
the others can't make progress, and the work is said to *starve* them.

The sample simulates slow work with `thread::sleep`, which blocks the whole thread:

```rust,ignore
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}
```

The two futures each call `slow` several times, and only then await a short timer:

```rust,ignore
let a = async {
    println!("'a' started.");
    slow("a", 30);
    slow("a", 10);
    slow("a", 20);
    trpl::sleep(Duration::from_millis(50)).await;
    println!("'a' finished.");
};
```

`b` is written the same way. The output is:

```
'a' started.
'a' ran for 30ms
'a' ran for 10ms
'a' ran for 20ms
'b' started.
'b' ran for 75ms
'b' ran for 10ms
'b' ran for 15ms
'b' ran for 350ms
'a' finished.
```

`a` does all of its slow work before it reaches its first await point, so `b` doesn't start until `a` has paused. Then `b` runs all of its slow work, and `a` finishes last. The two
futures never take turns.

The fix is to add await points between the slow steps, so the runtime can switch. Listing 17-16 in the book does that with `trpl::sleep`, which interleaves the work but makes the
program wait on timers. The next step uses `yield_now`, which hands control back without waiting.

---

### Try it in the runner

Press **Run**. The output should match the list above.

The comments in `src/main.rs` suggest moving the sleep to the start of each future. Work out the new output first. `a` starts and waits on its timer, so `b` runs next, and
its blocking work holds the thread for 450 ms. When `a` is polled again, its timer has already expired, so it runs its slow work and finishes. The expected output is:

```
'a' started.
'b' started.
'b' ran for 75ms
'b' ran for 10ms
'b' ran for 15ms
'b' ran for 350ms
'a' ran for 30ms
'a' ran for 10ms
'a' ran for 20ms
'a' finished.
```

---

*For more detail, see section 17.3 "More Futures" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch17-03-more-futures.html).*
