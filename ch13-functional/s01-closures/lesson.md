## 13.1 Closures

Closures are anonymous functions that can capture the values around them. They're used to customize behavior, and many standard library methods accept them.

* [Capturing the environment](#goto/01-capturing-the-environment): a closure that uses `self`, in `unwrap_or_else`.
* [Closure types and syntax](#goto/02-closure-types): annotated and inferred closures, and why types are inferred.
* [Capturing references or moving ownership](#goto/03-capturing-references): immutable and mutable capture, and `move`.
* [The Fn traits](#goto/04-fn-traits): `FnOnce`, `FnMut`, and `Fn`, and how they're used in the standard library.
* [Summary](#goto/05-summary): the main ideas of this sub-chapter.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
