## Chapter 15: Smart Pointers

A pointer is a variable that holds the address of other data. A reference, written `&`, is the most common kind. A *smart pointer* is a data structure that acts like a
pointer, and also carries extra information or capabilities. Many smart pointers own the data they point to, which a reference doesn't.

This chapter covers the most common smart pointers in the standard library, and the patterns they support. The chapter is split into sub-sections:

* [15.1 Using Box<T> to point to data on the heap](#goto/s01-box): storing a value on the heap, and enabling recursive types such as a cons list.
* [15.2 Treating smart pointers like regular references with Deref](#goto/s02-deref): following `*`, implementing `Deref`, and deref coercion.
* [15.3 Running code on cleanup with the Drop trait](#goto/s03-drop): the order of drops, and dropping a value early.
* [15.4 Rc<T>, the reference-counted smart pointer](#goto/s04-rc): sharing data between several owners, and the reference count.
* [15.5 RefCell<T> and interior mutability](#goto/s05-interior-mutability): borrowing rules checked at runtime, a mock object, and shared mutable data.
* [15.6 Reference cycles can leak memory](#goto/s06-reference-cycles): how a cycle keeps values alive, and how `Weak<T>` prevents it.

---

## Summary

Smart pointers give Rust programs control over ownership, mutation, and cleanup that references alone can't. `Box<T>` puts data on the heap, `Rc<T>` shares it, and `RefCell<T>`
allows changes through an immutable reference. `Deref` and `Drop` let smart pointers feel like regular values and clean up after themselves. `Weak<T>` keeps shared structures
from forming cycles that leak.

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
