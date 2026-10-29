## 10.2 Traits: Defining Shared Behavior

A trait describes behavior that a type can share with others. Generic code can then use trait bounds to require that behavior, instead of depending on a
particular type. This sub-chapter covers defining and implementing traits, default methods, trait bounds and `impl Trait`, returning traits, and conditional
and blanket implementations.

* [Defining and implementing a trait](#goto/01-defining-a-trait): a `Summary` trait, implemented by two types.
* [Default implementations](#goto/02-default-implementations): a method body that types can keep or override.
* [Traits as parameters](#goto/03-traits-as-parameters): `impl Trait`, trait bounds, and the fixed `largest` function.
* [Returning types that implement traits](#goto/04-returning-impl-trait): returning a value whose concrete type stays hidden.
* [Conditional and blanket implementations](#goto/05-conditional-and-blanket): methods that exist only for some types, and implementations for every type that qualifies.
* [Summary](#goto/06-summary): the main ideas of this sub-chapter.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
