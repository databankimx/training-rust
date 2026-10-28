## Summary: Trait Objects

A trait object lets one collection hold values of different types, as long as they share a trait.

* **Box<dyn Trait>:** a pointer to any value that implements the trait, decided at runtime. *(See the screen of different types step.)*
* **Mixed types:** a library can accept types it doesn't know about, as long as they implement the trait. *(See the screen of different types step.)*
* **Inference:** a vector of trait objects often needs a type annotation or a cast. *(See the type inference step.)*
* **Dispatch:** trait objects use dynamic dispatch, which costs a lookup at runtime. Generics use static dispatch, which is resolved at compile time. *(See the type inference step.)*

Trait objects can't be downcast to their concrete type, as some object-oriented languages allow. That's a deliberate limit of the design.

---

### Check your understanding

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Does this program compile? If not, on which line does Rust report the problem?

```rust
use std::fmt::Debug;
fn main() {
    let n = 1;
    let s = String::from("Hello");
    let v: Vec<&dyn Debug> = vec![&n, &s];
    let n_ref = v[0] as &i32;
    println!("{}", n_ref + 1);
}
```

○ This program compiles, and prints `2`
○ This program does not compile, on line 5
◉ This program does not compile, on line 6
○ This program does not compile, on line 7
→ Yes. `v[0]` is a `&dyn Debug`, and a trait object can't be cast back to a more specific type like `&i32`. Rust doesn't allow that cast, so line 6 is rejected. The `Any` trait is the exception.
````

````quiz
Consider implementing a generic function in two ways:

```rust
fn f_opt1<T: SomeTrait>(t: &T) { /* ... */ }
fn f_opt2(t: &dyn SomeTrait) { /* ... */ }
```

Which of the following is NOT a reason to prefer the trait object version (`f_opt2`) over the type-parametric version (`f_opt1`)?

○ Faster compile times
○ Smaller binary size
◉ Improved runtime performance
○ Better developer ergonomics
→ Yes. Trait objects avoid monomorphization, so they compile faster and produce smaller binaries. They usually run more slowly, because of the dynamic dispatch. Developer ergonomics are rarely better with trait objects, so that answer also isn't a reason to prefer them.
````

---

*The summary is written for this course, and draws on section 18.2 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch18-02-trait-objects.html).*
