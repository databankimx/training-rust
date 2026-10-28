## Supertraits

Sometimes a trait's default method needs a method from another trait. The trait it depends on is called a *supertrait*. You write it after a colon in the trait's definition:

```rust,ignore
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        // ...
    }
}
```

`OutlinePrint: fmt::Display` means that a type can implement `OutlinePrint` only if it also implements `Display`. That's why `self.to_string()` is available inside the default method. `to_string` comes from `Display`,
and the supertrait guarantees it exists for every implementor.

The sample prints a value inside a frame of asterisks. `Point` implements `Display` to show `(x, y)`, and then implements `OutlinePrint` with no methods of its own, because the default method is enough:

```rust,ignore
impl OutlinePrint for Point {}
```

The output is a box around the text:

```
**********
*        *
* (1, 3) *
*        *
**********
```

Without the supertrait, the default method would fail to compile, because `to_string` wouldn't be known for `&Self`. And a type that doesn't implement `Display` can't implement `OutlinePrint` at all. The
compiler reports that `Display` is required but not implemented.

---

### Try it in the runner

Press **Run**. The output should be the five lines above.

The comments in `src/main.rs` suggest removing `: fmt::Display` from the trait. The compiler then reports that no method named `to_string` is found for `&Self`, which is the error the supertrait prevents.

---

*For more detail, see section 20.2 "Advanced Traits" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html).*
