## The Static Lifetime

The `'static` lifetime means that a reference can live for the entire program. String literals have this lifetime, because the text is stored in the compiled
program, which is always available.

```rust,ignore
let s: &'static str = "I have a static lifetime.";
```

The sample prints this string, and the output is:

```
I have a static lifetime.
```

---

### Be careful with 'static

When an error message suggests `'static`, think before you add it. The suggestion usually appears because a reference is dangling, or because two lifetimes
don't match. Adding `'static` hides the problem rather than fixing it. The right fix is usually to make the data live long enough, or to return an owned value.

---

### Lifetimes and generics together

A function can take lifetimes, generic type parameters, and trait bounds at once. The sample's `longest_with_an_announcement` does all three:

```rust,ignore
fn longest_with_an_announcement<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
```

The lifetime `'a` and the type parameter `T` go in the same list inside the angle brackets. `T` must implement `Display`, because the announcement is printed with
`{ann}`. The function prints the announcement, then returns the longer of the two strings. The sample's output is:

```
Announcement! Today is a good day
The longest string is abcd
```

`abcd` is longer than `xyz`, so it's returned.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
I have a static lifetime.
Announcement! Today is a good day
The longest string is abcd
```

The comments in `src/main.rs` suggest passing a number as the announcement. The function still compiles, because numbers implement `Display`, and the trait bound is
the only thing the function asks of `T`.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
If a reference has a lifetime `'static`, what does this mean?

○ The data under the reference cannot be mutated
→ Not quite. Mutability is a separate question. A `'static` reference is usually immutable, but that isn't what the lifetime means.
○ The data under the reference lives in the static region of memory
→ Not quite. Where the data is stored isn't part of the meaning. The lifetime only says how long it's valid.
○ The data under the reference is not dynamic
→ Not quite. This is close, but the lifetime's meaning is about how long the data lives.
◉ The data under the reference is never deallocated
→ Yes. `'static` means the data lives for the whole program, so it's never freed while the reference exists.
````

---

*For more detail, see section 10.3 "Validating References with Lifetimes" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html).*
