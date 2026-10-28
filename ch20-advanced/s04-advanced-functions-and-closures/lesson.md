## 20.4 Advanced Functions and Closures

Functions and closures can be passed and returned in different ways. This sub-section covers function pointers, named functions used with iterator adapters, returning closures with `impl Fn`, and storing
several closures with `Box<dyn Fn>`.

* [Function pointers](#goto/01-function-pointers): `fn` parameters, `map` with `ToString::to_string`, and enum variants as functions.
* [Returning closures](#goto/02-returning-closures): `impl Fn` return types, and `Box<dyn Fn>` in a vector.
* [Summary](#goto/03-summary): the main ideas, and the book's quiz.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
