## Summary: Running Tests

`cargo test` compiles the code in test mode, and runs the tests in parallel. Its defaults keep the output readable, and options let you change them.

* **Output capture:** output from passing tests is hidden, and output from failing tests appears with the failure. `--show-output` shows it for passing tests too.
  *(See the output capture step.)*
* **Ignored tests:** `#[ignore]` keeps a slow test out of ordinary runs, and the summary still reports it. `--ignored` and `--include-ignored` run them.
  *(See the ignored tests step.)*
* **Parallel or single-threaded:** tests share a process, so tests that touch shared state can interfere. `--test-threads=1` runs them one at a time.
  *(See the options and filters step.)*
* **Filtering:** a name, or part of one, selects the tests to run. The module path is part of each test's name. *(See the options and filters step.)*

A test suite is only useful if it's quick enough to run often. Ignoring slow tests, and filtering to the part of the code you're changing, keep it fast. Run the full
suite, including the ignored tests, before you rely on it.

---

*The summary is written for this course, and draws on chapter 11 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch11-02-running-tests.html).*
