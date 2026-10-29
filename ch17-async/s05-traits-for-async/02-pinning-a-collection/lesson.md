## Pin and Unpin for a Collection of Futures

Chapter 17.3 used `join!` to wait for a fixed number of futures. A collection whose size is only known at runtime needs a `Vec` and `join_all`. That runs into two problems.

The first is that every async block has its own type, even when two blocks return the same thing. A `Vec` can hold only one type, so each future is turned into a trait object, `dyn Future<Output = ()>`.
Trait objects are covered in Chapter 18. The future is then stored in a `Box`, which puts it on the heap, and the type is written out in full:

```rust,ignore
let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![
    Box::pin(async { println!("one") }),
    Box::pin(async { println!("two") }),
    Box::pin(async { println!("three") }),
];
```

The second problem is `Pin`. An async block can hold references to its own local data. Moving such a future in memory would leave those references pointing at the old location. `Pin` promises the value won't move,
so the future is safe to poll. Trait objects that need this promise must be pinned, and `Box::pin` does that for the data on the heap.

If you use `Box::new` instead, the compiler reports that `Unpin` isn't implemented for `dyn Future<Output = ()>`. Most types implement `Unpin`, which means they can be moved freely, even when pinned.
A future from an async block may hold references to itself, so it doesn't implement `Unpin`, and it needs the `Pin` wrapper.

`join_all` then awaits the whole collection, and polls each future in the order it appears:

```
one
two
three
```

The `pin!` macro from step 1 pins a single value on the stack. `Box::pin` does the same for a value on the heap, which is what a `Vec` of futures needs.

---

### Try it in the runner

Press **Run**. The output should be `one`, `two`, and `three`, each on its own line.

The comments in `src/main.rs` suggest replacing `Box::pin` with `Box::new`, and changing the type to `Box<dyn Future>`. The compiler reports that `Unpin` is missing for the trait object, which is the error from the book.

---

*For more detail, see section 17.5 "A Closer Look at the Traits for Async" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch17-05-traits-for-async.html).*
