## 20.1 Unsafe Rust

Rust's compiler is conservative, and sometimes rejects code that is correct. Unsafe Rust lets you take responsibility for those cases, with five extra operations. This sub-section covers raw pointers and a
safe abstraction over them, the mutable static, and calling C.

* [Raw pointers and a safe split](#goto/01-raw-pointers-and-split): raw pointers, and a `split_at_mut` built on them.
* [Mutable static variables](#goto/02-mutable-static): a counter that needs `unsafe`, and `SAFETY` comments.
* [Calling C with extern](#goto/03-calling-c): an `unsafe extern` block, and a function marked `safe`.
* [Summary](#goto/04-summary): the main ideas, and the book's quizzes.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
