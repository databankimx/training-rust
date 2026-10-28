## Chapter 18: Object-Oriented Programming Features of Rust

Rust isn't a purely object-oriented language, but it has the features that matter most for object-oriented design. This chapter asks what those features are, and how Rust provides
them. It covers objects and encapsulation, trait objects for values of different types, and an object-oriented design pattern implemented in Rust.

The chapter is split into sections, as the book has them:

* [18.1 Characteristics of object-oriented languages](#goto/s01-what-is-oo): objects with methods, encapsulation with `pub`, and reusing behavior without inheritance.
* [18.2 Using trait objects that allow for values of different types](#goto/s02-trait-objects): `Box<dyn Trait>`, type inference with trait objects, and dynamic dispatch.
* [18.3 Implementing an object-oriented design pattern](#goto/s03-oo-design-patterns): the state pattern with trait objects, and encoding states as types.
* [18.4 Ownership inventory #4](#goto/s04-ownership-inventory): lifetimes and trait objects in a realistic function, with three quizzes.
* [18.5 Design trade-offs](#goto/s05-design-challenge): four design case studies, each with a requirement and a yes-or-no quiz for each design.

Chapter 18 is now complete.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
