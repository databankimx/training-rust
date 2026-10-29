## Summary: Test Organization

Rust has two kinds of tests, and each has its place.

* **Unit tests:** small, and kept in the same file as the code, inside a `tests` module marked `#[cfg(test)]`. They can check private functions.
  *(See the unit tests step.)*
* **Integration tests:** in the `tests` folder, where each file is its own crate. They use only the public interface, and check that the parts work together.
  *(See the integration tests step.)*
* **Shared helpers:** put them in `tests/common/mod.rs`, so they aren't treated as a test crate. *(See the shared helpers and binaries step.)*
* **Binary crates:** keep the logic in `src/lib.rs`, so integration tests can reach it. *(See the shared helpers and binaries step.)*

Unit tests check the pieces, and integration tests check how the pieces fit together. Both matter. Even with Rust's type system and ownership rules, tests are still the
way to check that code behaves as intended.

---

*The summary is written for this course, and draws on the summary of chapter 11 in [The Rust Programming Language](https://doc.rust-lang.org/book/ch11-03-test-organization.html).*
