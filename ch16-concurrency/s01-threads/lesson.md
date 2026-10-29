## 16.1 Using Threads to Run Code Simultaneously

Threads let a program run parts of its work at the same time. This sub-section creates threads with `thread::spawn`, waits for them with `join`, and shows why `move` is needed
when a thread uses values from the code around it.

* [Spawning threads and waiting with join](#goto/01-spawning-threads): running a spawned thread, and the difference `join` makes to the order of output.
* [Using move closures with threads](#goto/02-move-closures): transferring ownership of a value into a thread, with the book's quizzes.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
