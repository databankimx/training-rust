## 17.3 More Futures

Futures can pause and resume only at await points, and combining them builds new behavior. This sub-section shows what happens when a future doesn't yield, how `yield_now` fixes it,
and how `select` and `sleep` make a timeout.

* [Starving the runtime](#goto/01-starving-the-runtime): one future blocks the thread, so the other can't run.
* [Yielding with yield_now](#goto/02-yield-now): handing control back without waiting on a timer.
* [Building a timeout](#goto/03-timeout): `select` and `sleep` combined into a new function.
* [Summary](#goto/04-summary): the main ideas, and the book's quizzes.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
