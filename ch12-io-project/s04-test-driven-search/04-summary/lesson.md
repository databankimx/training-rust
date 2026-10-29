## Summary: Test-Driven Search

The search was built test-first, one small step at a time.

* **Failing test:** the test describes the behavior, and the first version of `search` is written only to make it compile. The failure confirms the test works.
  *(See the writing a failing test step.)*
* **Passing test:** `search` checks each line with `contains`, and the test passes. *(See the writing code to pass the test step.)*
* **Using search:** `run` prints the matching lines, which is what the user asked for. *(See the using search in run step.)*

Because the logic is in the library, the test calls `search` directly, with no files and no command line. The binary only parses arguments and prints results.

---

*The summary is written for this course, and draws on the summary of section 12.4 in [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html).*
