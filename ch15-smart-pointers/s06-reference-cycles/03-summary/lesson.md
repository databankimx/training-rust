## Summary: Smart Pointers

Smart pointers let you choose guarantees and trade-offs that differ from Rust's defaults for references.

* **Box<T>:** stores data on the heap, with a known size. It makes recursive types possible. *(See the 15.1 steps.)*
* **Deref and Drop:** `Deref` lets a smart pointer be used like a reference, and `Drop` runs code when a value goes out of scope. *(See the 15.2 and 15.3 steps.)*
* **Rc<T>:** lets several owners share data, counted at runtime. *(See the 15.4 steps.)*
* **RefCell<T>:** allows mutation through an immutable reference, with the borrowing rules checked at runtime. Combined with `Rc<T>`, it allows shared mutable data.
  *(See the 15.5 steps.)*
* **Reference cycles:** `Rc` references that point at each other keep one another alive, so the memory leaks. `Weak<T>` breaks a cycle by referring without owning.
  *(See the 15.6 steps.)*

Memory leaks are possible in safe Rust, and that's a deliberate boundary rather than a bug in the language. The tools in this chapter let you reason about ownership, so a cycle
is a choice you make, not something that happens unnoticed.

Chapter 16 covers concurrency, and introduces smart pointers that work across threads.

---

*The summary is written for this course, and draws on the summary of chapter 15 in [The Rust Programming Language](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html).*
