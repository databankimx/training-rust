## 18.2 Using Trait Objects That Allow for Values of Different Types

A vector holds one type. Trait objects let it hold several types that share a trait, which suits a library that must work with types its authors haven't written yet. This sub-section builds a
screen of components, fixes the type inference problem that trait objects cause, and ends with the book's quizzes.

* [A screen of different types](#goto/01-screen-of-different-types): `Box<dyn Draw>`, and a type added outside the library.
* [Type inference with trait objects](#goto/02-type-inference-with-trait-objects): the annotation the compiler needs, and dynamic dispatch.
* [Summary](#goto/03-summary): the main ideas, and the book's quizzes.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
