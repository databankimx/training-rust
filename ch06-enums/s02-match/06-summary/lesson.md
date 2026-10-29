## Summary: The match Control Flow Construct

`match` compares a value with a series of patterns, and runs the code for the first pattern that matches. It's an expression, so it
produces a value.

* **Match expressions:** arms are tried from the top, and the first one that matches is used. *(See the match expressions step.)*
* **Binding values:** a pattern can bind a name to the data inside a variant, and the arm can then use it. *(See the binding values step.)*
* **Matching Option:** `Some` and `None` are handled with `match` like any other enum. Because `Option<T>` and `T` are different types, the compiler
  requires both cases to be handled. *(See the matching Option step.)*
* **Exhaustive matches and catch-all patterns:** every `match` must cover every possible value. A catch-all, a name or `_`, covers the rest, and it must
  come last. *(See the exhaustive matches step.)*
* **Ownership in match:** matching a value can move its data, and matching a reference borrows it instead. Matching on `&opt` is the usual way to look
  inside an enum without taking its contents. *(See the ownership in match step.)*

The compiler checks each `match` against the rules of ownership and exhaustiveness. A program that compiles has handled every case, and hasn't moved
data it still needs.

The next section covers `if let`, a shorter form for when you only care about one pattern.

---

*The summary is adapted from the section 6.2 summary in [The Rust Programming Language](https://doc.rust-lang.org/book/ch06-02-match.html).*
