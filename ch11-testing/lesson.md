## Chapter 11: Writing Automated Tests

Tests show that code does what it should, and they keep it that way as the code changes. Rust has tests built into the language, run with `cargo test`.
This chapter covers how to write tests, how to run them and choose which ones run, and how to organize them in a project.

The chapter is split into sub-chapters:

* [11.1 Writing tests](#goto/s01-writing-tests): the `#[test]` attribute, the assert macros, `should_panic`, and tests that return a `Result`.
* [11.2 Running tests](#goto/s02-running-tests): output capture, ignored tests, and choosing which tests run.
* [11.3 Test organization](#goto/s03-test-organization): unit tests, integration tests, shared helpers, and binary crates.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
