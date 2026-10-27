## Chapter 10: Generic Types, Traits, and Lifetimes

This chapter covers three tools for writing code that works in many situations. Generics let one definition serve many types. Traits describe the
behavior a type must have, so generic code can rely on it. Lifetimes tell the compiler how long references must stay valid.

The chapter is split into sub-chapters:

* [10.1 Generic data types](#goto/s01-generic-data-types): generic structs, enums, and methods, and why they cost nothing at runtime.
* [10.2 Traits](#goto/s02-traits): defining shared behavior, default methods, trait bounds, `impl Trait`, and blanket implementations.
* [10.3 Lifetimes](#goto/s03-lifetimes): preventing dangling references, annotating functions and structs, elision, and `'static`.
* [10.4 Ownership inventory #3](#goto/s04-inventory): a generic function that sorts a borrowed slice, and a method whose helper borrows all of `self`.

The chapter's material is complete.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
