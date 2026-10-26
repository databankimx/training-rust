## Summary: Concise Control Flow

`match` handles every case of an enum, and the compiler checks that you covered them all. Sometimes you only care about one case, or one case
must hold for the rest of the function. Rust has two shorter forms for those situations.

* **`if let`:** handles one pattern, and ignores the rest. It's shorter than a `match` with an empty `_` arm, but it doesn't check exhaustively.
  *(See the if let step.)*
* **`if let` with `else`:** adds a branch for the values that don't match, in the same way as a `_` arm. *(See the if let with else step.)*
* **`let...else`:** binds a pattern for the rest of the function, and returns early when the pattern doesn't match. It keeps the main work on the top
  level. *(See the let...else step.)*

Choose the construct by asking how many cases matter, and what each one should do:

* Every case needs its own result: use `match`.
* One case does something, and the rest can be ignored: use `if let`.
* One case is required, and the rest should stop the function: use `let...else`.

Enums, `match`, and these shorter forms together let you express the possible states of a value in your domain, and handle each one clearly. That's
the main idea of Chapter 6.

---

*The summary is adapted from the section 6.3 summary in [The Rust Programming Language](https://doc.rust-lang.org/book/ch06-03-if-let.html).*
