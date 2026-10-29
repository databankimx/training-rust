## Summary: Advanced Functions and Closures

Functions and closures can be passed and returned in several ways, and each has a different trade-off.

* **Function pointers:** the `fn` type accepts named functions, and closures without captures, and is needed when a caller can only take a plain function. *(See the function pointers step.)*
* **Named functions in adapters:** `map` accepts a function or an enum variant constructor, as well as a closure. *(See the function pointers step.)*
* **Returning `impl Fn`:** one closure can be returned without naming its type. *(See the returning closures step.)*
* **Returning `Box<dyn Fn>`:** closures with different types can share a collection, at the cost of a heap allocation and a dynamic call. *(See the returning closures step.)*

Prefer generic parameters with `Fn` bounds when the function should accept closures too. Use `fn` when only plain functions can be given.

---

### Check your understanding

*This question is adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Consider implementing a `register` function that takes a callback in two ways:

```rust,ignore
fn register1(cb: fn(Event) -> ());
fn register2<F>(cb: F) where F: Fn(Event) -> ();
```

Which signature lets `register` accept the widest variety of arguments?

○ `register1`
◉ `register2`
○ They are equivalent
→ Yes. Closures that capture values can be passed to `register2`. Only plain functions, or closures without captures, can be passed to `register1`, because `fn` can't hold any captured state.
````

---

*The summary is written for this course, and draws on section 20.4 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch20-04-advanced-functions-and-closures.html).*
