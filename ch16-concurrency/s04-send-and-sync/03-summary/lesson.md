## Summary: Send and Sync

Many concurrency features live in the standard library, not the language. Two marker traits are the exception. They're part of the language, and the compiler uses them to decide what
can cross between threads.

* **Send:** ownership of a value can move to another thread. Most types are `Send`. *(See the first step.)*
* **Sync:** a value can be used from several threads at once. A type is `Sync` when a reference to it is `Send`. *(See the first step.)*
* **Some types are one but not the other:** `RefCell<T>` is `Send`, but not `Sync`. `Rc<T>` is neither. `Mutex<T>` is both. *(See the first step.)*
* **Design choices:** a type can be made `Send` but not `Sync` to allow moves and forbid sharing, as with the connection in the second step.

You don't implement `Send` or `Sync` for types made from parts that already have them, because the compiler works that out. Implementing them by hand requires `unsafe` code, which
Chapter 20 covers.

---

*The summary is written for this course, and draws on the summary of section 16.4 in [The Rust Programming Language](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html).*
