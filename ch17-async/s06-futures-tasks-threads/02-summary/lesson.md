## Summary: Futures, Tasks, and Threads

Threads and async are two approaches to concurrency, and they often work best together.

* **Threads:** managed by the operating system, and suited to CPU-bound work that splits into parts. *(See the threads and async together step.)*
* **Tasks:** managed by the runtime, and cheap enough to create in large numbers. They suit I/O-bound work, where messages arrive at unpredictable times. *(See the threads and async together step.)*
* **Together:** a thread can do blocking work and send results through a channel, while async code receives them. *(See the threads and async together step.)*

Chapter 21 applies these ideas to a larger project, and compares the thread and task approaches directly.

---

*The summary is written for this course, and draws on section 17.6 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch17-06-futures-tasks-threads.html).*
