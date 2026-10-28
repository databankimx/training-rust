## Chapter 17: Fundamentals of Asynchronous Programming: Async, Await, Futures, and Streams

Many operations spend most of their time waiting, for a network, a file, or a timer. Asynchronous programming lets a program do other work during that wait. This chapter builds
on the threads of Chapter 16, and introduces futures, streams, and the `async` and `await` syntax, with a runtime that runs them.

The chapter is split into sections, as the book has them:

* [17.1 Futures and the async syntax](#goto/s01-futures-and-syntax): calling async functions, `block_on`, and racing two futures with `select`.
* [17.2 Applying concurrency with async](#goto/s02-concurrency-with-async): `join`, async channels, and closing a channel with `move`.
* [17.3 More futures](#goto/s03-more-futures): starving the runtime, `yield_now`, and building a timeout from `select` and `sleep`.
* [17.4 Streams: futures in sequence](#goto/s04-streams): creating a stream from an iterator, and using `filter` and `take`.
* [17.5 A closer look at the traits for async](#goto/s05-traits-for-async): `poll`, `Pin`, `Unpin`, and `Stream`.
* [17.6 Putting it all together: futures, tasks, and threads](#goto/s06-futures-tasks-threads): when to use threads, async, or both.

---

## Summary

Async code runs many waiting operations on one or a few threads. Futures describe work that isn't ready yet, `await` polls them until they are, and a runtime runs the
tasks that hold them. Streams bring the same idea to sequences of values. Threads and tasks are complementary: threads suit CPU-bound work, and async suits I/O-bound work,
and a program can use both.

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
