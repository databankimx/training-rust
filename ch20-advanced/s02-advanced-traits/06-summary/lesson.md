## Summary: Advanced Traits

Traits have more features than the basic form covered in Chapter 10.

* **Associated types:** a trait can name a placeholder type that each implementation fixes once, such as `Item` in `Iterator`. *(See the associated types step.)*
* **Default type parameters:** `Rhs = Self` lets an operator trait default to the same type on both sides, and be customized when needed. *(See the operator overloading step.)*
* **Disambiguation:** when two methods share a name, use the trait name, or fully qualified syntax, `<Type as Trait>::function`. *(See the disambiguating names step.)*
* **Supertraits:** a trait can require another trait, so its default methods can use the other trait's methods. *(See the supertraits step.)*
* **Newtypes:** a single-field tuple struct lets you implement a trait the orphan rule would otherwise forbid. *(See the newtype step.)*

---

### Check your understanding

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Recall the definition of the `Add` trait:

```rust,ignore
trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}
```

Why is `Output` an associated type, while `Rhs` is a type parameter?

○ The add operation takes `Rhs` as input and produces `Output` as output, and type parameters are used for input while associated types are used for output
○ A trait can only have a single associated type, so `Rhs` must be a type parameter to `Add`
◉ A type `T` should be addable to many other types `S`, but a given `T + S` operation should always have a single output type
○ An associated type cannot have a default, while a trait type parameter can have a default
→ Yes. One type can be added to several right-hand types, so `Rhs` is a parameter. For each pair, the output is fixed, so `Output` is an associated type.
````

````quiz
Why is `Rhs` a type parameter of the trait `Add`, rather than a type parameter of the function `add`?

○ The type-checker executes more efficiently when `Rhs` is a trait-level type parameter than a function-level type parameter
○ `Rhs` cannot have a default type as a function-level type parameter
○ Code generation for the `add` function would require additional time for monomorphization due to the function-level type parameter
◉ If `Rhs` were a function-level type parameter, then the definition of `add` could not assume any structure to `Rhs`
→ Yes. With `Rhs` at the function level, `add` would have to work for any type, so it couldn't rely on anything about the right-hand side. At the trait level, each implementation knows its `Rhs`.
````

Does this program compile? If so, what does it print?

```rust,ignore
mod inner {
    pub trait A {
        fn f(&self) -> usize { 0 }
    }
    pub trait B {
        fn f(&self) -> usize { 1 }
    }
    pub struct P;
    impl A for P {}
    impl B for P {}
}
fn main() {
    use inner::{P, B};
    println!("{}", P.f());
}
```

````quiz
Program result

○ This program does not compile
○ This program compiles, and prints `0`
◉ This program compiles, and prints `1`
→ Yes. Only `B` is in scope, so `P.f()` refers to the method from `B`, which returns 1. When only one of two conflicting traits is imported, Rust uses that one, and fully qualified syntax isn't needed.
````

````quiz
Consider implementing a trait `Trait` for a type `T`. In which situation do you need to wrap `T` in a newtype?

○ `Trait` is defined in the local crate and `T` is defined in an external crate
○ `Trait` is defined in an external crate and `T` is defined in the local crate
○ `Trait` is defined in the local crate and `T` is defined in the local crate
◉ `Trait` is defined in an external crate and `T` is defined in an external crate
→ Yes. The orphan rule forbids this combination, because neither the trait nor the type is local. A newtype makes the type local, so the implementation becomes allowed.
````

---

*The summary is written for this course, and draws on section 20.2 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html).*
