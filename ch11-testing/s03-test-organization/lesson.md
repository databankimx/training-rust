## 11.3 Test Organization

The Rust community thinks about tests in two categories: unit tests, which check one module in isolation, and integration tests, which use the library from outside.
This sub-chapter covers where each kind goes, how `cfg(test)` keeps test code out of the build, how to share helpers between integration tests, and how
binary crates fit in.

* [Unit tests](#goto/01-unit-tests): the `tests` module, `cfg(test)`, and testing private functions.
* [Integration tests](#goto/02-integration-tests): the `tests` folder, and the three sections of `cargo test` output.
* [Shared helpers and binary crates](#goto/03-shared-helpers-and-binaries): `tests/common/mod.rs`, and why binaries keep their logic in a library.
* [Summary](#goto/04-summary): the main ideas of this sub-chapter.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
