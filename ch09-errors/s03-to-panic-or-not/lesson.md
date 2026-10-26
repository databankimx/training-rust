## 9.3 To panic! or Not to panic!

Deciding between `panic!` and `Result` is a design choice. This sub-chapter explains the situations where each is right, and how a type can make a check
happen once, instead of in every function.

* [When panic is fine](#goto/01-when-panic-is-fine): examples, prototypes, tests, and values you know are valid.
* [Custom types](#goto/02-custom-types): a checked constructor and a private field, so that valid values are guaranteed.
* [Summary](#goto/03-summary): the main ideas of Chapter 9.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
