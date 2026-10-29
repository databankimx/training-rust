## Summary: Recoverable Errors with Result

Errors that the program can respond to are returned as a `Result`, with an `Ok` value on success and an `Err` value that says why it failed. The caller
decides what to do.

* **Result and match:** a `match` handles both cases. The error says why an operation failed, which `Option` can't express. *(See the result and match step.)*
* **Matching on errors:** a nested `match` on `ErrorKind` handles one reason, and decides what to do with the rest. *(See the matching on errors step.)*
* **unwrap and expect:** both return the value inside `Ok`, or panic on `Err`. `expect` takes a message that explains the expectation. *(See the unwrap and expect step.)*
* **Propagating with ?:** `?` returns an error to the caller, and gives the value inside `Ok` otherwise. It works only in functions whose return type matches. *(See
  the propagating step.)*
* **main returning Result:** `main` can return `Result<(), Box<dyn Error>>`, so `?` works at the top level, and an error sets a nonzero exit code. *(See the
  main returns a result step.)*

The rule of thumb is to return an error when the caller might handle it, and to panic only when the program can't continue. Section 9.3 covers that choice
in detail.

---

*The summary is adapted from the closing discussion of section 9.2 in [The Rust Programming Language](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html).*
