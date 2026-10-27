## Summary: Performance

Closures and iterators are high-level features, and they don't cost anything at runtime.

* **Zero-cost abstractions:** a chain of iterator adapters compiles to roughly the same code as a hand-written loop. You pay no runtime penalty for the clearer version.
  *(See the loops and iterators timed step.)*
* **Measuring:** compare optimized builds, and repeat each measurement. A debug build says more about the build settings than about the code.
  *(See the loops and iterators timed step.)*

Closures and iterators express high-level ideas, and the compiler keeps the performance low-level. That's part of Rust's aim of providing zero-cost abstractions, so you
can use them without worrying about speed.

---

*The summary is written for this course, and draws on the summary of section 13.4 in [The Rust Programming Language](https://doc.rust-lang.org/book/ch13-04-performance.html).*
