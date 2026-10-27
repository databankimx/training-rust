## Defining and Implementing a Trait

A trait defines behavior that different types can share. It lists method signatures, and each type that implements the trait supplies its own body for
them. Code that works with the trait can then call those methods on any type that implements it.

A trait is similar to an interface in other languages, with some differences that the next steps cover.

---

### Defining the trait

The sample defines a `Summary` trait with one method:

```rust,ignore
pub trait Summary {
    fn summarize(&self) -> String;
}
```

The signature ends with a semicolon, because the trait doesn't provide a body. Each type that implements `Summary` must provide one, with this exact
signature. The compiler checks that it does.

---

### Implementing the trait

The sample has two types, a news article and a social post. Each one implements the trait with `impl Summary for`:

```rust,ignore
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

After `impl`, you write the trait's name, then `for`, then the type. Inside the block, each method appears with its real body. The sample's output is:

```
New article: Penguins win the Stanley Cup Championship!, by Iceburgh (Pittsburgh, PA, USA)
New post: horse_ebooks: of course, as you probably already know, people
```

Both types expose `summarize`, so the calling code is the same for either one. The types also keep their own fields. The sample leaves out the book's extra
fields, such as a post's reply flag, because nothing reads them, and the compiler would warn about them.

---

### Traits and the orphan rule

You can implement a trait on a type only if the trait or the type is defined in your crate. For example, a crate can implement its own `Summary` for `Vec<T>`,
because the trait is local. It can't implement the standard library's `Display` for `Vec<T>`, because both are external. The quiz in the next step checks this rule.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
New article: Penguins win the Stanley Cup Championship!, by Iceburgh (Pittsburgh, PA, USA)
New post: horse_ebooks: of course, as you probably already know, people
```

The comments in `src/main.rs` suggest removing `summarize` from `SocialPost`'s impl block. The compiler reports that `SocialPost` doesn't implement `Summary`,
because a required method is missing.

---

*For more detail, see section 10.2 "Defining Shared Behavior with Traits" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html).*
