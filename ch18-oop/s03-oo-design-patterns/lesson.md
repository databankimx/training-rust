## 18.3 Implementing an Object-Oriented Design Pattern

The state pattern describes how a value changes behavior as its state changes. This sub-section builds a blog post with three states, first with trait objects in the object-oriented style, and then
with each state encoded as a type, which moves some checks to compile time.

* [The state pattern with trait objects](#goto/01-state-pattern-with-trait-objects): `State`, `Box<dyn State>`, and transitions that consume the old state.
* [Encoding states as types](#goto/02-states-as-types): one type per state, and transitions that take a value and return the next type.
* [Summary](#goto/03-summary): the two versions compared, and the book's quiz.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
