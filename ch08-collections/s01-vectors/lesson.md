## 8.1 Storing Lists of Values with Vectors

A vector, `Vec<T>`, holds a list of values of one type, stored next to each other in memory. This sub-chapter covers how to create and change
a vector, how to read its elements safely, how to iterate over them, and how an enum lets one vector hold different kinds of value.

* [Creating and updating](#goto/01-creating-and-updating): `Vec::new`, `vec!`, and `push`.
* [Reading elements](#goto/02-reading-elements): indexing against `get`, and what happens when an index is out of range.
* [Iterating](#goto/03-iterating): `for` loops over shared and mutable references, and why you can't change a vector while looping over it.
* [Enums and dropping](#goto/04-enums-and-dropping): one vector of mixed types, and what happens when a vector is dropped.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
