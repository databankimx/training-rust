## Controlling Access with a Mutex

A mutex, short for *mutual exclusion*, allows only one thread at a time to use some data. A thread asks for the mutex's lock before it touches the data, and releases the lock
when it's done. The data is said to be guarded by the mutex.

Mutexes are easy to get wrong when they're managed by hand. Forgetting to unlock stops every other thread for good. Rust's type system helps here. In `Mutex<T>`, the data
lives inside the mutex, so the only way to reach it is through `lock`:

```rust,ignore
let m = Mutex::new(5);

{
    let mut num = m.lock().unwrap();
    *num = 6;
}

println!("m = {}", *m.lock().unwrap());
```

`lock` blocks the current thread until the lock is free, and returns a `Result`. The `unwrap` panics if a thread that held the lock panicked earlier, which the book calls
*poisoning*. The value that `lock` returns is a `MutexGuard`. It implements `Deref`, so `*num` reaches the `i32` inside, and it implements `Drop`. When the guard goes out of scope,
the lock is released automatically. That's why the braces matter: they end the first guard's life before the second `lock` runs.

The output is:

```
m = 6
```

If the first guard were still alive at the second `lock`, the program would wait for a lock that never gets released, and hang. `Mutex<T>` doesn't prevent that case, because a
deadlock is a logic error, not a memory error.

---

### Try it in the runner

Press **Run**. The output should be `m = 6`.

The comments in `src/main.rs` suggest printing the mutex with `{:?}`. The Debug output shows the data, and whether the mutex is poisoned. The exact format changes between Rust
versions, so compare with the lesson in general terms only.

---

*For more detail, see section 16.3 "Shared-State Concurrency" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch16-03-shared-state.html).*
