## The Newtype Pattern

The *orphan rule* says a trait can be implemented for a type only when the trait or the type is defined in your crate. `Display` belongs to the standard library, and so does `Vec<T>`. So you can't implement
`Display` for `Vec<String>` directly.

The newtype pattern gets around this. A *newtype* is a tuple struct with one field, which wraps the type you need. The new struct is defined in your crate, so the orphan rule doesn't stop you from implementing a trait on it:

```rust,ignore
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
```

`self.0` reaches the vector inside the wrapper. The output of the sample is:

```
w = [hello, world]
```

The newtype has no runtime cost. The wrapper is removed at compile time, so the vector is stored exactly as before.

**The downside.** A `Wrapper` doesn't have the methods of the `Vec` it holds, so `w.len()` doesn't compile. You have two options. You can write each method you need, so it delegates to `self.0`. Or you can implement `Deref`,
as Chapter 15.2 showed, which makes the wrapper act like the vector for method calls. Choosing not to forward some methods is also a way to restrict what the wrapper can do.

---

### Try it in the runner

Press **Run**. The output should be `w = [hello, world]`.

The comments in `src/main.rs` suggest calling `w.len()`. The compiler reports that no method named `len` exists for `Wrapper`. Adding `Deref<Target = Vec<String>>` to `Wrapper` would make the call work.

---

*For more detail, see section 20.2 "Advanced Traits" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html).*
