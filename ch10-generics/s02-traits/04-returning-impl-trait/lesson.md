## Returning Types That Implement Traits

`impl Trait` also works in return position. The function promises that it returns some type that implements the trait, and the caller can use only the
methods of that trait. The concrete type stays hidden.

---

### The sample

The function `returns_summarizable` returns a `SocialPost`, but its signature says only `impl Summary`:

```rust,ignore
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    }
}
```

The caller calls `summarize` on the result, and it works because `Summary` is the only interface the caller needs. The output is:

```
horse_ebooks: of course, as you probably already know, people
```

This is useful when the concrete type is long or complicated, as with iterators and closures, which Chapter 13 covers. The caller doesn't need to name it.

---

### One type only

A function with `impl Trait` in return position must return one concrete type. A function that returns a `NewsArticle` in one branch and a `SocialPost` in
another doesn't compile, because `impl Summary` names one type, and the compiler must know which. Returning values of different types needs trait objects,
which Chapter 18 covers.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
horse_ebooks: of course, as you probably already know, people
```

The comments in `src/main.rs` suggest changing the return type to `SocialPost`. The output is the same, but the caller now depends on the concrete type.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Does this program compile? If not, on which line does Rust report the problem?

```rust
use std::fmt::Display;
fn displayable<T: Display>(t: T) -> impl Display { t }

fn main() {
  let s = String::from("hello");
  let mut s2 = displayable(s);
  s2.push_str(" world");
  println!("{s2}");
}
```

○ This program compiles
→ Not quite. `displayable` returns `impl Display`, so the caller only knows the value implements `Display`.
◉ This program does not compile, on line 7
→ Yes. `s2` is only known to implement `Display`, so `push_str`, a method of `String`, can't be called on it.
○ This program does not compile, on line 2
→ Not quite. The function itself is fine. The problem is the caller's use of `push_str`, on line 7.
````

---

*For more detail, see section 10.2 "Defining Shared Behavior with Traits" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html).*
