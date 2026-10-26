## Chapter 9: Error Handling

Rust separates errors into two kinds. Some are unrecoverable, and the program can't continue safely. Others are recoverable, and the program can
report them and carry on. This chapter covers both, and how to choose between them.

The chapter is split into sub-chapters:

* [9.1 Unrecoverable errors with panic!](#goto/s01-unrecoverable-errors): the `panic!` macro, panics from code, and backtraces.
* [9.2 Recoverable errors with Result](#goto/s02-recoverable-errors): matching on `Result`, `unwrap` and `expect`, the `?` operator, and `main` returning a `Result`.
* [9.3 To panic or not to panic](#goto/s03-to-panic-or-not): when a panic is the right choice, and custom types that guarantee valid values.

That completes the chapter's material.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
