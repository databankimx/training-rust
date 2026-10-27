## Lifetimes in Structs and Elision

A struct that holds a reference needs a lifetime on each reference it holds. This step covers that, and the rules the compiler uses to fill in lifetimes when you
leave them out.

---

### A struct that holds a reference

The sample defines `ImportantExcerpt`, which holds a string slice. The struct declares a lifetime, and uses it on the field:

```rust,ignore
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

An `ImportantExcerpt` can't outlive the text its `part` refers to. The sample takes the first sentence of a `String`, and wraps it in an excerpt. The `String`
lives until the end of `main`, so the excerpt is valid.

---

### Methods on the struct

Lifetimes on the struct's type are declared after `impl`, and then used after the struct's name. The sample's method has two references as inputs, `&self` and
`announcement`. The third elision rule gives the output the lifetime of `&self`, so the method needs no annotation:

```rust,ignore
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}
```

The output is:

```
Attention please: Listen up
Call me Ishmael
```

---

### The elision rules

Rust doesn't require every lifetime to be written out. It applies three rules to fill in the missing ones:

* **Rule 1:** each input reference gets its own lifetime parameter.
* **Rule 2:** if there is exactly one input lifetime, it is given to every output lifetime.
* **Rule 3:** if one of the inputs is `&self` or `&mut self`, its lifetime is given to every output lifetime.

If the rules still leave a lifetime unknown, the compiler reports an error and asks for an annotation. It won't guess. That's why `longest` from the last step needed
its annotation: two inputs and no `self` leave the output's lifetime undecided.

The quizzes below apply the rules to two signatures. The first has a struct with a lifetime, passed by value. The second passes a reference to it.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Attention please: Listen up
Call me Ishmael
```

The comments in `src/main.rs` suggest changing the method to return a string literal, with the announcement typed as `&'static str`. The output is the same,
because the method no longer depends on the struct's text.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Consider the following un-annotated function signature.

```rust
struct Foo<'a> {
  bar: &'a i32
}

fn baz(f: Foo) -> &i32 { /* ... */ }
```

Will Rust accept this function signature? If so, what lifetimes will it infer?

○ Rust will reject this function signature
→ Not quite. Rule 1 gives `Foo` a lifetime, and Rule 2 applies, because there is exactly one input lifetime.
○ `fn baz(f: Foo) -> &i32`
→ Not quite. Rust does fill in the lifetimes, so the signature is written out in full.
○ `fn baz<'a>(f: Foo<'a>) -> &i32`
→ Not quite. The output reference still needs a lifetime, and Rule 2 gives it one.
○ `fn baz<'a>(f: Foo<'a>) -> &'a i32`
→ Not quite. This is close, but the output needs its lifetime written in the right place, and the inputs must not be `Foo` without one.
◉ `fn baz<'a>(f: Foo<'a>) -> &'a i32`
→ Yes. Rule 1 gives `Foo` the lifetime `'a`, and Rule 2 gives that same lifetime to the output, because there is exactly one input lifetime.
````

````quiz
Consider the following un-annotated function signature.

```rust
struct Foo<'a> {
  bar: &'a i32
}

// Foo changed to &Foo
fn baz(f: &Foo) -> &i32 { /* ... */ }
```

Will Rust accept this function signature? If so, what lifetimes will it infer?

○ `fn baz(f: &Foo) -> &i32`
→ Not quite. Written out like this, the output has no lifetime tied to its source.
○ `fn baz<'a>(f: &Foo<'a>) -> &'a i32`
→ Not quite. This would be valid if it were written, but the rules don't pick it.
○ `fn baz<'a>(f: &'a Foo) -> &'a i32`
→ Not quite. This is a possible annotation, but Rust doesn't infer it.
◉ Rust will reject this function signature
→ Yes. There are two input lifetimes, one for `&Foo` and one for the `i32` inside `Foo`, and neither rule settles the output. Rust won't guess which one the output comes from.
````

---

*For more detail, see section 10.3 "Validating References with Lifetimes" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html).*
