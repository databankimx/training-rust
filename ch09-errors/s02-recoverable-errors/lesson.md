## 9.2 Recoverable Errors with Result

Most errors can be handled by the program, and `Result` is how Rust represents them. This sub-chapter covers how to handle a `Result` with `match`,
how to choose between `unwrap` and `expect`, how the `?` operator passes errors upward, and how `main` can return a `Result` too.

* [Result and match](#goto/01-result-and-match): the `Result` enum, and why it's used instead of `Option`.
* [Matching on errors](#goto/02-matching-on-errors): handling one kind of error, and panicking on the rest.
* [unwrap and expect](#goto/03-unwrap-and-expect): shortcuts for a value you expect to have, and messages that explain why.
* [Propagating with ?](#goto/04-propagating-with-question-mark): returning errors to the caller, and where `?` can be used.
* [main returns a Result](#goto/05-main-returns-result): using `?` in `main`, and the exit code.
* [Summary](#goto/06-summary): the main ideas of this sub-chapter.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
