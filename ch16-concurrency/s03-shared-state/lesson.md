## 16.3 Shared-State Concurrency

Threads can also communicate by sharing memory, where every thread can reach the same data. This is safe only when access is controlled. This sub-section covers the
mutex, which lets one thread at a time use the data, and `Arc<T>`, which lets several threads own it.

* [Controlling access with a mutex](#goto/01-mutex-one-thread): `lock`, the guard, and releasing the lock.
* [Shared ownership of a mutex with Arc](#goto/02-shared-counter): ten threads updating one counter, and why `Rc<T>` isn't enough.
* [Summary](#goto/03-summary): the main ideas, and the book's quizzes.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
