## Summary: Writing Tests

A test is a function marked `#[test]`. It sets up its data, runs the code, and checks the result. `cargo test` runs every test and reports the outcome.

* **Writing a test:** `#[test]` marks the function, and `assert_eq!` checks two values for equality. Tests usually live in a `tests` module marked `#[cfg(test)]`.
  *(See the writing a test step.)*
* **assert!:** checks that a condition is `true`. Test the false case as well as the true one. *(See the checking with assert step.)*
* **assert_eq! and assert_ne!:** compare two values, and print both when they differ. A custom message explains the reason for a check. *(See the assert_eq and
  messages step.)*
* **should_panic:** the test passes only if the code panics, and `expected` checks that the message is the one you want. *(See the should_panic and result step.)*
* **Result in tests:** a test can return `Result` and use `?`. It can't also be `should_panic`. *(See the should_panic and result step.)*

Tests are most useful when they fail for the right reason. A good test checks a behavior you care about, and a failing message should tell you what went wrong.
The next sub-chapter covers how to run tests, select some of them, and ignore others.

---

*The summary is written for this course, and draws on chapter 11 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch11-01-writing-tests.html).*
