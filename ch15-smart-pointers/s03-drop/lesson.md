## 15.3 Running Code on Cleanup with the Drop Trait

The `Drop` trait runs code when a value goes out of scope, and Rust calls it automatically. This is how smart pointers free their data, and how your own types can release
files, locks, and connections. This sub-section shows when `drop` runs, and how to drop a value early.

* [The Drop trait](#goto/01-drop-trait): implementing `drop`, and the order in which values are dropped.
* [Dropping a value early](#goto/02-early-drop): `std::mem::drop`, and the book's quizzes.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
