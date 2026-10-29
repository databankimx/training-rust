## Traits as Parameters

Traits become most useful when a function accepts any type that implements them. Two syntaxes do this, and this step covers both. It also returns to the
`largest` function from the last sub-chapter, which didn't compile without a trait bound.

---

### impl Trait

A parameter declared with `impl Trait` accepts any type that implements the trait:

```rust,ignore
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

Inside the function, you can call any method from `Summary` on `item`. Passing a type that doesn't implement `Summary`, such as a `String`, is a compile
error. The sample's output for the article is:

```
Breaking news! Penguins win the Stanley Cup Championship!, by Iceburgh
```

---

### Trait bounds

`impl Trait` is shorthand for a trait bound on a generic parameter. The longer form is the same function:

```rust,ignore
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

The two forms differ when there are two parameters. `fn notify(a: &impl Summary, b: &impl Summary)` lets the two arguments have different types, as long as
both implement `Summary`. `fn notify<T: Summary>(a: &T, b: &T)` forces both to be the same type.

Several bounds are written with `+`, as in `T: Summary + Display`. A `where` clause moves the bounds after the signature, which is easier to read when a function
has many of them.

---

### Fixing largest

The `largest` function from the last sub-chapter compares its items with `>`. Not every type supports that, so the function needs a bound:

```rust,ignore
fn largest<T: PartialOrd>(list: &[T]) -> &T {
```

`PartialOrd` is the trait that provides `>` and the other comparison operators. With the bound, the function works for any type that can be compared, and the
caller can't pass a type that can't. The sample's output is:

```
The largest number is 100
The largest char is y
```

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Breaking news! Penguins win the Stanley Cup Championship!, by Iceburgh
The largest number is 100
The largest char is y
```

The comments in `src/main.rs` suggest calling `largest` with a slice of `NewsArticle` values. `NewsArticle` doesn't implement `PartialOrd`, so the compiler rejects
the call, and the bound is what makes it reject the call.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
What is the smallest set of trait bounds on `T` needed to make this function type-check?

```rust
fn f<T: /* ??? */>(t: &T) {
  let t2 = t.clone();
  println!("{t2}");
}
```

○ No bounds
→ Not quite. `clone` and `{}` both require a bound, because `T` has no methods of its own.
○ `Clone`
→ Not quite. Clone is needed for `clone`, but `println!` with `{}` also needs `Display`.
◉ `Clone + Display`
→ Yes. `t.clone()` needs `Clone`, and printing with `{}` needs `Display`.
○ `Clone + Display + Debug`
→ Not quite. This works, but `Debug` isn't needed, because `{}` uses `Display`, not `{:?}`. The question asks for the smallest set.
````

````quiz
Which of these is NOT permitted by Rust's rules for implementing traits?

○ Implementing an external trait for a local type
→ Not quite. This is permitted, because the type is local to your crate.
○ Implementing a local trait for an external type
→ Not quite. This is permitted, because the trait is local to your crate.
○ Implementing a local trait for a local type
→ Not quite. This is permitted, because both are local.
◉ Implementing an external trait for an external type
→ Yes. Neither the trait nor the type is local, so the orphan rule forbids it. Otherwise two crates could provide conflicting implementations.
````

---

*For more detail, see section 10.2 "Defining Shared Behavior with Traits" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html).*
