## 11.1 Writing Tests

Tests are ordinary Rust functions, marked with an attribute. This sub-chapter covers how to write them, how to check results with `assert!`, `assert_eq!`, and
`assert_ne!`, how to check for panics with `should_panic`, and how to write a test that returns a `Result`.

* [Writing a test](#goto/01-writing-a-test): the `#[test]` attribute, and `assert_eq!` in a tests module.
* [Checking with assert!](#goto/02-checking-with-assert): testing both the true and the false case of a `bool` result.
* [assert_eq! and messages](#goto/03-assert-eq-and-messages): comparing values, and adding a message that explains a check.
* [should_panic and Result](#goto/04-should-panic-and-result): expected panic messages, and tests that return a `Result`.
* [Summary](#goto/05-summary): the main ideas of this sub-chapter.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
