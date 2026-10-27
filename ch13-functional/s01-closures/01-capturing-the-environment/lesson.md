## Capturing the Environment

Closures are anonymous functions. They can be saved in a variable or passed to another function, and unlike functions, they can use values from the place where
they're defined. That's what makes them useful for customizing behavior.

The sample is a T-shirt giveaway. A company has an `Inventory` of shirts, and a winner may have a favorite color. If the winner has no favorite, they get the color the
company has the most of. The `giveaway` method decides which color to give:

```rust,ignore
fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
    user_preference.unwrap_or_else(|| self.most_stocked())
}
```

`unwrap_or_else` returns the value inside `Some`. For `None`, it calls the closure it's given, and returns what the closure returns. The closure `|| self.most_stocked()`
takes no arguments, and it uses `self`. It captures a reference to the inventory from the place where it's defined.

Functions can't do this. A function can use only what it's given as arguments, or what's global. A closure can use the variables around it, and the standard library
didn't need to know anything about `Inventory` to call it.

The program prints:

```
The user with preference Some(Red) gets Red
The user with preference None gets Blue
```

The second user has no preference, so the closure runs. The inventory has two blue shirts and one red, so the most stocked color is Blue.

---

### Try it in the runner

Press **Run**. The output should match the lines above.

The comments in `src/main.rs` suggest changing the inventory to two red shirts and one blue. Think about what the second user gets before you run it.

---

*For more detail, see section 13.1 "Closures: Anonymous Functions That Capture Their Environment" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch13-01-closures.html).*
