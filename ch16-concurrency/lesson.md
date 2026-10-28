## Chapter 16: Fearless Concurrency

Concurrency runs parts of a program at the same time. Rust's ownership and type rules catch many of the problems that come with it, at compile time. This chapter covers
threads, message passing between them, shared state, and the two traits that decide what can cross between threads.

The chapter is split into sub-sections:

* [16.1 Using threads to run code simultaneously](#goto/s01-threads): spawning threads, waiting for them with `join`, and `move` closures.

Later sections will cover message passing, shared state with `Mutex<T>` and `Arc<T>`, and the `Send` and `Sync` traits.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
