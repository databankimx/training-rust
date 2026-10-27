## 10.1 Generic Data Types

Generics let a struct, enum, function, or method work with many types, without writing each version by hand. This sub-chapter covers generic structs with one
and two type parameters, methods that are generic or that apply to one type, and why generics cost nothing at runtime.

* [Generic structs](#goto/01-generic-structs): one parameter for a struct whose fields share a type, and two parameters when they don't.
* [Generic methods](#goto/02-generic-methods): methods for every type, and methods that exist only for one concrete type.
* [Monomorphization](#goto/03-monomorphization): how the compiler turns generic code into specific code, with no runtime cost.
* [Summary](#goto/04-summary): the main ideas of this sub-chapter.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
