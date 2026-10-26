## 4.4 Slices

A slice is a reference to a contiguous part of a collection, rather than the whole thing. Slices solve a real problem: returning
part of a string without having to keep a separate index in sync with it.

* [The index problem](#goto/01-the-index-problem): why returning a position is fragile.
* [String slices](#goto/02-string-slices): the `&str` type, and how slices are represented.
* [Slices and mutation](#goto/03-slices-and-mutation): how a slice keeps its data borrowed.
* [Slices as parameters](#goto/04-slices-as-parameters): writing functions that accept any kind of text.
* [Other slices](#goto/05-other-slices): slices of arrays and other collections.
* [Summary](#goto/06-summary): the main ideas of this sub-chapter.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
