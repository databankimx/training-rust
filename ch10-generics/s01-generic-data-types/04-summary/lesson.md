## Summary: Generic Data Types

Generics let you write a definition once, and use it with many types. A type parameter stands for a type the compiler fills in where the code is used.

* **Generic structs and enums:** a struct or enum can take one or more type parameters, in angle brackets after its name. Fields that share a parameter must
  have the same type. *(See the generic structs step.)*
* **Generic methods:** `impl<T>` makes methods available for every type. `impl Point<f32>` makes methods available for one concrete type only. *(See the generic
  methods step.)*
* **No runtime cost:** the compiler generates a specific version of generic code for each type it's used with, so the program runs as fast as hand-written code.
  *(See the monomorphization step.)*

A generic type knows nothing about the types it stands for, so a generic function can only do what every type allows. To do more, such as printing or comparing,
a generic function has to say which capabilities it needs. That's the subject of the next sub-chapter, traits.

---

*The summary is adapted from the discussion of generic data types in [The Rust Programming Language](https://doc.rust-lang.org/book/ch10-01-syntax.html).*
