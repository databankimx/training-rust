## Reusing Behavior Without Inheritance

Inheritance is one of the features often associated with object-oriented languages. A child class gets its parent's data and methods, and can override some of the methods. Rust
doesn't have inheritance, but it does reuse behavior, in two ways. This step shows the first, default methods in traits.

The trait `Summary` requires `author`, and provides a default for `summarize`:

```rust,ignore
pub trait Summary {
    fn author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.author())
    }
}
```

`Post` implements only `author`, so it gets the default `summarize`. `Tweet` overrides `summarize`, the way a child class overrides an inherited method. Both types can be passed to a
function that accepts any `Summary`, and the function calls whichever `summarize` applies:

```rust,ignore
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

The output is:

```
(Read more from Alice...)
@bob: Trait objects are handy
Breaking news! @bob: Trait objects are handy
```

The default behavior is shared, so `Post` doesn't repeat it. The override gives `Tweet` its own version, and the code that uses `Summary` doesn't need to know which one it has.

Rust's version is more limited than inheritance in one way. A trait's default method can call only the other methods of that trait, not fields of the type. That's why `summarize` calls
`author`. Trait objects, which the next section covers, are the other way Rust reuses behavior.

---

### Try it in the runner

Press **Run**. The output should match the three lines above.

The comments in `src/main.rs` suggest deleting the `summarize` method from `Tweet`. Two lines change, the `Tweet` line and the `notify` line, and both now show the default text, `(Read more from @bob...)`.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Which of the following aspects of object-oriented programming does Rust NOT implement?

○ Objects with methods
○ Encapsulation of private data
◉ Inheritance of superclass methods
○ Every aspect listed here is implemented
→ Yes. Rust has objects, and encapsulation through `pub`, but it has no inheritance. Default trait methods share behavior, but a struct can't inherit fields or methods from another struct.
````

---

*For more detail, see section 18.1 "Characteristics of Object-Oriented Languages" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch18-01-what-is-oo.html).*
