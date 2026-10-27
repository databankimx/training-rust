## 10.3 Validating References with Lifetimes

Every reference has a lifetime, the span of the program during which it's valid. This sub-chapter covers why lifetimes matter, how the borrow checker uses them,
how to annotate functions and structs, how the elision rules fill in the common cases, and the special `'static` lifetime.

* [Dangling references](#goto/01-dangling-references): a reference must not outlive its data.
* [Lifetimes in functions](#goto/02-longest-with-lifetimes): annotating `longest`, and what the annotation promises.
* [Structs and elision](#goto/03-structs-and-elision): references inside structs, and the three elision rules.
* [The static lifetime and generics](#goto/04-static-and-generics): `'static` references, and lifetimes alongside type parameters and bounds.
* [Summary](#goto/05-summary): the main ideas of this sub-chapter.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
