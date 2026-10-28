## 15.2 Treating Smart Pointers Like Regular References with Deref

The `Deref` trait lets a type customize the dereference operator, `*`. A smart pointer that implements `Deref` can be used like a reference, and code written for references
works with it too. This sub-section compares `*` on a reference and a box, builds a `MyBox<T>` type, and shows deref coercion.

* [Following references and boxes](#goto/01-following-references): `*` on a reference and on a `Box`.
* [MyBox, Deref, and deref coercion](#goto/02-mybox-and-deref-coercion): implementing `Deref`, and calling a function with `&MyBox<String>`, with the book's quiz.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
