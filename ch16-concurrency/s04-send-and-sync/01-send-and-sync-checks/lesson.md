## Send and Sync

Two marker traits in the standard library describe what can happen across threads. They have no methods, and their job is to let the compiler enforce concurrency rules.

* **Send** means a value's ownership can be moved to another thread. Almost every type is `Send`. A type made entirely of `Send` parts is automatically `Send` too.
* **Sync** means a value can be referred to from several threads at once. A type `T` is `Sync` when `&T` is `Send`. Again, most types are `Sync`, and a type made entirely of `Sync` parts
  is `Sync` too.

A type can be one, both, or neither. The standard library's examples are the reasons the compiler rejects certain code:

* `Rc<T>` is neither `Send` nor `Sync`, because its count is updated without protection. That's why Listing 16-14 failed, with the message that `Rc<Mutex<i32>>` cannot be sent between threads.
* `RefCell<T>` is `Send`, so it can be moved to another thread, but not `Sync`. Its borrow checks at runtime aren't thread-safe, so two threads can't share it.
* `Mutex<T>` is both `Send` and `Sync`, which is why `Arc<Mutex<T>>` works across threads.

The sample checks these facts at compile time. Each function takes a type parameter with a bound, and calling it with a type that lacks the trait is a compile error:

```rust,ignore
fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

assert_sync::<RefCell<i32>>();
```

The sample's output is:

```
Arc<Mutex<i32>> is Send and Sync
Mutex<i32> is Send and Sync
RefCell<i32> is Send
```

The check lines compile, because each type has the trait they require. The line that would fail is commented out.

---

### Try it in the runner

Press **Run**. The output should match the three lines above.

The comments in `src/main.rs` suggest uncommenting the `RefCell` line for `Sync`. The compiler rejects it, because `RefCell<i32>` isn't `Sync`. Rust reports that the trait bound `Sync` isn't satisfied.

---

*For more detail, see section 16.4 "Extensible Concurrency with the Sync and Send Traits" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html).*
