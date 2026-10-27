## Summary: Publishing to Crates.io

A crate is documented, tested, organized for its users, and then published.

* **Documentation comments:** `///` documents the item below it, and `//!` documents the item that contains it, such as the crate. *(See the doc comments and doc tests step.)*
* **Doc tests:** the examples in documentation are run by `cargo test`, so they can't go out of date. *(See the doc comments and doc tests step.)*
* **Public API:** `pub use` re-exports items at a convenient path, without changing the internal layout. *(See the public API step.)*
* **Publishing:** a crate needs a unique name, a description, and a license before `cargo publish` accepts it. A published version is permanent, and `cargo yank` stops new
  projects from using a broken version. *(See the publishing step.)*

Documentation is part of the code. When the examples run as tests, the documentation stays correct as the code changes.

---

### Check your understanding

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Say you are publishing a library, and you want to write documentation about the entire library at the top of the `lib.rs` file. The documentation should be prefixed with
the following comment syntax:

◉ `//!`
→ Yes. `//!` documents the item that contains it, such as the crate, and `///` documents the item below it.
○ `///`
→ Not quite. `///` documents the item that follows it, such as a function, not the whole library.
○ `//`
→ Not quite. `//` is an ordinary comment, and it doesn't appear in the documentation.
````

````quiz
Say you are creating a library `worldly` with the following function in `lib.rs`. Its documentation example claims the result is "Hello world", but the function only adds
"world":

```rust
/// Adds the string "world" to `s`.
///
/// # Example
/// ```
/// let mut s = String::new();
/// worldly::add_world(&mut s);
/// assert_eq!(s, "Hello world");
/// ```
pub fn add_world(s: &mut String) {
    s.push_str("world");
}
```

If you run `cargo test`, will any test fail?

○ No
◉ Yes
→ Yes. The example runs as a doc test, and its `assert_eq!` expects "Hello world", but the function produces "world", so the doc test fails.
````

````quiz
Say you are publishing a library `foobar v0.1.0`. After running `cargo publish`, you realize there is a bug in one of your functions. Which of the following best describes how to
overwrite the published crate at the same `v0.1.0` version?

○ Add the flag `--overwrite-published` to the `cargo publish` command
○ Login to Crates.io and delete `v0.1.0` of the published crate
○ Run `cargo yank` and then run `cargo publish`
◉ It is impossible to overwrite a published crate at a given version
→ Yes. Crates.io doesn't allow a published version to be overwritten. Yank the buggy version, then publish a new one, such as `0.1.1`.
````

````quiz
Which of the following steps is NOT required to publish a crate to Crates.io?

○ Adding a license to the `[package]` section of `Cargo.toml`
○ Registering and logging in to Crates.io
○ Selecting a crate name that is not registered on Crates.io
◉ Adding documentation to each public function in the crate
→ Yes. Documentation is strongly recommended, but publishing doesn't require it. A library can add `#![deny(missing_docs)]` if it wants missing documentation to be an error.
````

---

*The summary is written for this course, and draws on the quizzes in section 14.2 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html).*
