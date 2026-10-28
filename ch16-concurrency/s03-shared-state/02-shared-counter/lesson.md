## Shared Ownership of a Mutex with Arc

Sometimes several threads need to own the same mutex. Chapter 15 gave a value several owners with `Rc<T>`, so that's the first thing to try. It doesn't work across threads.
The compiler reports that `Rc<Mutex<i32>>` cannot be sent between threads safely, because `Rc<T>` updates its count without any protection. Two threads changing the count at the
same moment could leave it wrong, which can free a value too early or leak it.

`Arc<T>` is the thread-safe version. The *A* stands for *atomic*: its count changes with atomic operations, which can't be interrupted by another thread. It has the same API as
`Rc<T>`, so changing to it takes only a different `use` line, and the calls to `new` and `clone` become `Arc`. The cost is a little extra work on each count change, so use `Rc<T>`
on one thread, and `Arc<T>` only across threads.

This sample has ten threads. Each one clones the `Arc`, locks the mutex, and adds one:

```rust,ignore
let counter = Arc::new(Mutex::new(0));

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}
```

Each thread's guard is dropped when its closure ends, so the next thread can lock the mutex. The main thread joins all ten threads before it reads the total. The output is:

```
Result: 10
```

The `Mutex` makes each increment happen one at a time, so no increment is lost. Without it, two threads could read the same value and both write back the same result.

The `Mutex<T>` here guards an integer. For simple numbers like this one, the standard library's `std::sync::atomic` types are simpler and faster. The book uses a mutex so the lesson stays on how the mutex works.

---

### Try it in the runner

Press **Run**. The output should be `Result: 10`.

The comments in `src/main.rs` suggest changing the number of threads to 100. The result should then be 100, every time, because the mutex keeps every increment.

---

*For more detail, see section 16.3 "Shared-State Concurrency" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch16-03-shared-state.html).*
