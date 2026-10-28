## 15.1 Using Box<T> to Point to Data on the Heap

A box is the simplest smart pointer. It stores its value on the heap, and it's most useful when the size of a type has to be known at compile time, as in a recursive type.

* [Storing data on the heap](#goto/01-storing-on-the-heap): `Box::new`, and what happens when a box goes out of scope.
* [Enabling recursive types](#goto/02-cons-list): a cons list whose `Cons` holds a box, and a recursive function that walks it, with the book's quizzes.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
