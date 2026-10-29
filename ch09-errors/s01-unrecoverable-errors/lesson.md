## 9.1 Unrecoverable Errors with panic!

Some errors leave the program with no safe way forward. Rust's answer is the `panic!` macro, which stops the program with a message, and cleans up
as it goes. This sub-chapter covers calling `panic!` directly, and panics that come from the code, along with how to find their cause.

* [The panic! macro](#goto/01-panic-macro): calling `panic!`, the message it prints, and what it does to the program.
* [Panics from code](#goto/02-panics-from-code): reading past the end of a vector, and tracking the cause with a backtrace.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
