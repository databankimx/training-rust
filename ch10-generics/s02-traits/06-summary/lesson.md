## Summary: Traits

A trait defines behavior that many types can share. Generic code uses trait bounds to say which behavior it needs, and the compiler checks each type that
is used with it.

* **Defining and implementing:** a trait lists method signatures, and each type implements them with `impl Trait for Type`. *(See the defining a trait step.)*
* **Default implementations:** a trait method can have a body, which types can keep or override. *(See the default implementations step.)*
* **Traits as parameters:** `impl Trait` and trait bounds accept any type with the behavior. A bound like `PartialOrd` makes `largest` work for every comparable
  type. *(See the traits as parameters step.)*
* **Returning impl Trait:** a function can return a value of a type it doesn't name, as long as it's always the same type. *(See the returning impl Trait step.)*
* **Conditional and blanket implementations:** an `impl` block can apply only when a bound is met, and a trait can be implemented for every type that meets a
  bound. *(See the conditional and blanket step.)*

The orphan rule keeps trait implementations coherent: a trait or its type must be local, so two crates can't provide conflicting implementations.

---

*The summary is adapted from the discussion of traits in [The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html).*
