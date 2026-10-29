## 20.2 Advanced Traits

Traits have features beyond the basic form. This sub-section covers associated types, default type parameters and operator overloading, disambiguating methods with the same name, supertraits, and the newtype
pattern.

* [Associated types](#goto/01-associated-types): the `Iterator` trait, and why it isn't written with a generic parameter.
* [Operator overloading and default type parameters](#goto/02-operator-overloading): `Add`, `Rhs = Self`, and adding millimeters to meters.
* [Disambiguating between methods with the same name](#goto/03-disambiguating-names): `Pilot`, `Wizard`, `Dog`, and fully qualified syntax.
* [Supertraits](#goto/04-supertraits): a trait that requires `Display`.
* [The newtype pattern](#goto/05-newtype): wrapping a type to get around the orphan rule.
* [Summary](#goto/06-summary): the main ideas, and the book's quizzes.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
