## 11.2 Running Tests

`cargo test` runs tests in parallel, captures their output, and reports the results. This sub-chapter covers what that means in practice: which output you see, how
to ignore slow tests, when to run tests one at a time, and how to choose which tests run.

* [Output capture](#goto/01-output-capture): what a failing test's output shows, and how passing output is hidden.
* [Ignored tests](#goto/02-ignored-tests): leaving out slow tests from ordinary runs.
* [Options and filters](#goto/03-options-and-filters): single-threaded runs, running tests by name, and terminal commands.
* [Summary](#goto/04-summary): the main ideas of this sub-chapter.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
