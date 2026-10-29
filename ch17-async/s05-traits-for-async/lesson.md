## 17.5 A Closer Look at the Traits for Async

The `Future`, `Stream`, and `StreamExt` traits are what async code is built from. Most of the time they stay out of sight, but error messages mention `Pin` and `Unpin`, and
a runtime or a custom future needs the details. This sub-section covers `poll`, pinning, and the `Stream` trait.

* [The poll method](#goto/01-the-poll-method): what `await` does, with a future that is pending several times.
* [Pinning a collection](#goto/02-pinning-a-collection): trait objects, `Box::pin`, and why `Unpin` matters for `join_all`.
* [Summary](#goto/03-summary): the main ideas, and the book's quizzes.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
