## 17.2 Applying Concurrency with Async

Async can handle the same concurrency problems as threads. This sub-section runs two tasks together, passes messages between them, and compares the results with the
threads of Chapter 16.

* [Joining two tasks](#goto/01-join-two-tasks): `trpl::join`, its fair polling, and the output order.
* [Sending data between tasks](#goto/02-async-channel): an async channel, `while let`, and closing it with `move`.
* [Summary](#goto/03-summary): the main ideas, and the book's remaining quizzes.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
