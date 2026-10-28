## Chapter 16: Fearless Concurrency

Concurrency runs parts of a program at the same time. Rust's ownership and type rules catch many of the problems that come with it, at compile time. This chapter covers threads,
message passing between them, shared state, and the two traits that decide what can cross between threads.

The chapter is split into sub-sections:

* [16.1 Using threads to run code simultaneously](#goto/s01-threads): spawning threads, waiting for them with `join`, and `move` closures.
* [16.2 Transfer data between threads with message passing](#goto/s02-message-passing): channels, the receiver as an iterator, and multiple producers.
* [16.3 Shared-state concurrency](#goto/s03-shared-state): `Mutex<T>` for one thread at a time, and `Arc<T>` for shared ownership across threads.
* [16.4 Extensible concurrency with Send and Sync](#goto/s04-send-and-sync): the two marker traits that the compiler uses to check what crosses threads.

---

## Summary

Rust makes concurrency safer without hiding it. Threads run work at the same time, and `join` waits for them. Channels move ownership of values from one thread to another. Mutexes
guard shared data, and `Arc<T>` lets threads share ownership of it. `Send` and `Sync` let the compiler refuse unsafe sharing before the program runs, so many data races never reach
production code.

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
