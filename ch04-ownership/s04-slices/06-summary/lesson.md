## Summary: The Slice Type

Slices let you refer to a contiguous part of a collection, rather than the whole collection. A slice is a kind of
reference, so it's a non-owning pointer.

Here's what the section covered, step by step:

* **The index problem:** returning a position, such as the end of the first word, leaves a number that has to be kept
  in sync with the data. Once the data changes, the number is stale. *(See the index problem step.)*
* **String slices:** a `&str` is a reference to part of a `String`, written with a range. Its length travels with its
  pointer. *(See the string slices step.)*
* **Slices and mutation:** a slice borrows the data it refers to, so clearing the string while a slice is still in use is
  rejected by the compiler. *(See the slices and mutation step.)*
* **Slices as parameters:** a `&str` parameter accepts a slice of a `String`, a reference to a `String`, and a string
  literal, because all of them are slices. *(See the slices as parameters step.)*
* **Other slices:** the same idea works for arrays and other collections, with the type `&[T]`. *(See the other slices step.)*

One advantage of slices over index-based positions is that a slice cannot go out of sync with the data it refers to,
while it's in use. The compiler checks that for you.

---

*The summary is adapted from the section 4.4 summary in [The Rust Programming Language](https://doc.rust-lang.org/book/ch04-04-slices.html).*
