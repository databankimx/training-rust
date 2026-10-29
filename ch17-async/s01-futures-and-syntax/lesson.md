## 17.1 Futures and the Async Syntax

Async code starts with futures, and the `async` and `await` keywords that work with them. This sub-section writes the first async functions, runs them with a runtime, and races two of them.

* [Calling an async function](#goto/01-hello-async): futures are lazy, and `block_on` runs them.
* [Racing two futures](#goto/02-racing-two-futures): `select` and `Either`, with the book's quizzes.

**A note before you start:** these steps use the `trpl` crate, which pulls in a number of other crates, including the Tokio runtime. The first run of a step downloads and builds all of them, so it can take a minute or more, and the Output pane may show "Compiling" or "Downloading" lines for a while. Later runs are quick, because the downloads are cached.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
