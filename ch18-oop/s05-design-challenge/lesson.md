## 18.5 Design Trade-offs

Knowing how Rust works isn't enough for a good design. You also have to decide which of Rust's tools suit a job. This sub-section gives four case studies. Each one describes a
set of designs, and a requirement, and asks whether each design meets it.

The book asks you to select every design that satisfies each requirement. Here, each design has its own question, so every question has one answer.

* [References](#goto/01-references): returning an asset from a manager, and what each choice allows.
* [Trait trees](#goto/02-trait-trees): how a widget's children are typed.
* [Dispatch](#goto/03-dispatch): how parallel and sequential callbacks are registered.
* [Intermediates](#goto/04-intermediates): serializing through an intermediate value, or directly.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
