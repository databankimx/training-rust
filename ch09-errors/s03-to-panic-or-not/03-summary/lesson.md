## Summary: Error Handling

Rust's error handling helps you write reliable code. `panic!` signals a state the program can't handle, and stops it, rather than letting it continue with
invalid values. `Result` says that an operation can fail in a way the caller can recover from, and gives the caller the choice.

* **Recoverable errors:** return a `Result` when failure is expected, such as bad user input, or a file that might not exist. The caller decides. *(See the
  recoverable errors section.)*
* **Unrecoverable errors:** panic when a contract is broken, or a state should never happen. The program stops with a message that names the problem.
  *(See the unrecoverable errors section.)*
* **Examples, prototypes, and tests:** `unwrap` and `expect` are appropriate, because they mark places to handle errors later, and they fail tests. *(See the
  when panic is fine step.)*
* **Custom types:** a type with a private field and a checked constructor lets the rest of your code rely on valid values, without checking them again.
  *(See the custom types step.)*

A good default is to return a `Result` from any function that might fail. Panic when the caller has done something wrong, or when continuing would be unsafe.

With Result and panic in hand, you have the tools to handle failures in the code you write. The next chapter introduces generics, which let you write code
that works with many types.

---

*The summary is adapted from the Summary of Chapter 9 in [The Rust Programming Language](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html).*
