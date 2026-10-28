## 17.6 Putting It All Together: Futures, Tasks, and Threads

Threads and async are complementary. This sub-section compares what each one manages, and shows a program that uses both: a thread for blocking work, and an async channel to
receive its results.

* [Threads and async together](#goto/01-threads-and-async-together): a thread sends numbers, and the async side receives them.
* [Summary](#goto/02-summary): the chapter as a whole.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
