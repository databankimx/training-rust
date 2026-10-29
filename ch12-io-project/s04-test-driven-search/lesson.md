## 12.4 Test-Driven Development of the Search

The search is the heart of `minigrep`, and it lives in the library, where it can be tested without running the program. This sub-chapter builds it test-first: write
a test that fails, make it pass with the simplest code that works, and then connect it to `run`.

* [Writing a failing test](#goto/01-failing-test): the test, and a first `search` that compiles but returns nothing.
* [Writing code to pass the test](#goto/02-passing-test): iterating over lines, and keeping the ones that match.
* [Using search in run](#goto/03-search-in-run): printing the matching lines.
* [Summary](#goto/04-summary): the main ideas of this sub-chapter.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
