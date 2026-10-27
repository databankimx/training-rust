## Summary: Improving the I/O Project

The `minigrep` code now uses iterators in three places, and each change made the code shorter and clearer.

* **Config::build:** takes ownership of an iterator of arguments, so the strings move into `Config` rather than being cloned. *(See the config with iterators step.)*
* **search:** a chain of `lines`, `filter`, and `collect` says what the function does, with no mutable vector. *(See the search with adapters step.)*
* **search as an iterator:** the function returns an iterator, so `run` can print each match as soon as it's found. *(See the returning an iterator step.)*

Iterators let the code describe the result it wants, and leave the steps to the compiler. The book's next section asks whether these versions are as fast as the loops
they replaced, and that's the subject of the next section.

---

*The summary is written for this course, and draws on the summary of section 13.3 in [The Rust Programming Language](https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html).*
