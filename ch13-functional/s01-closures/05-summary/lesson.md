## Summary: Closures

Closures are anonymous functions that can capture values from the place they're defined.

* **Capturing:** a closure uses `self`, or any local variable, without it being passed in. Functions can't do this. *(See the capturing the environment step.)*
* **Syntax and types:** closures can be written with or without type annotations. Their types are inferred from the first use, and the compiler then holds them to that
  type. *(See the closure types step.)*
* **Capture modes:** a closure borrows immutably, borrows mutably, or takes ownership, based on what its body does. `move` forces ownership. *(See the capturing references
  step.)*
* **Fn traits:** `FnOnce`, `FnMut`, and `Fn` describe how often a closure can be called, and whether it changes what it captures. A function that accepts a closure should
  ask for the least it needs. *(See the Fn traits step.)*

The next section covers iterators. Many iterator methods take closures, so these ideas come up again.

---

*The summary is written for this course, and draws on the summary of section 13.1 in [The Rust Programming Language](https://doc.rust-lang.org/book/ch13-01-closures.html).*
