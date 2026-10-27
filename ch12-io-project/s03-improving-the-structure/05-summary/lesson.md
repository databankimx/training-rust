## Summary: Improving the Structure

The program started with `main` doing everything. Four small steps changed that.

* **Config:** the arguments are grouped into a struct with named fields, so their purpose is clear. *(See the grouping the configuration step.)*
* **Result from build:** bad arguments are returned as an error, and `main` reports them and exits with code 1. *(See the returning a result step.)*
* **run:** the work moves into a function that returns a `Result`, and `?` passes errors up to `main`. *(See the moving the logic step.)*
* **Library crate:** the logic lives in `src/lib.rs`, where it can be tested, and `main.rs` only handles the arguments and the errors. *(See the library crate
  step.)*

A good `main` is short, and reads like a list of steps: parse the configuration, run the program, and report any error. The next sub-chapter writes the search
itself, and tests it.

---

*The summary is written for this course, and draws on the summary of section 12.3 in [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html).*
