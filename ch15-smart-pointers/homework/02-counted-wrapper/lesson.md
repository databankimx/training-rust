## Homework: A Counted Wrapper

**The problem:** Write a smart pointer, `Logged<T>`, that wraps a value and reports its life. When one is created, it prints a message. When one is dropped, it prints
another. Because it implements `Deref`, it can be used like the value inside it.

Create a project in VS Code with `cargo new logged`. In `src/main.rs`, write:

* A struct `Logged<T>` with two fields: `value: T`, and `name: &'static str`, used in the messages.
* `Logged::new(name: &'static str, value: T) -> Logged<T>`, which prints `created <name>` and returns the wrapper.
* An implementation of `Deref`, with `Target = T`, so that `*logged` and method calls such as `logged.len()` reach the inner value.
* An implementation of `Drop`, which prints `dropped <name>`.

In `main`, create two wrappers, a `String` named `first` and a `Vec<i32>` named `second`. Print the length of each through the wrapper, without writing `.value`. Then let
`main` end.

Write a test in a `tests` module that checks the wrapper behaves like its inner value, using `*` and a method call.

---

### Hints

* `Deref` needs a `deref` method that returns `&T`. It can return `&self.value`.
* Method calls like `first.len()` work through `Deref`, because the compiler follows the deref chain to find `len`.
* Values are dropped in the reverse order they were created. Think about which message comes first at the end of `main`.

---

### Check your work

Your program should print:

```
created first
created second
first has 5 bytes
second has 3 items
dropped second
dropped first
```

The `second` wrapper is dropped first, because it was created last.

---

### Extensions

* Add a `into_inner(self) -> T` method that returns the value and prints nothing. How do you stop `Drop` from running? (Think about moving `self` out of the wrapper.)
* Make `DerefMut` also work, so that `first.push_str(" world")` changes the inner `String`. What does the wrapper need to implement?

---

*This exercise is not from the book. It practices `Deref`, `Drop`, and smart pointers from Chapter 15.*
