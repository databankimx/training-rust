## Conditional and Blanket Implementations

An `impl` block can be restricted to types that satisfy some trait bounds. A method then exists only when the type parameter meets those bounds. A trait can
also be implemented for every type that meets a bound, which is called a blanket implementation.

---

### Conditional methods

The sample defines `Pair<T>` with two values of the same type. `new` works for any `T`:

```rust,ignore
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
```

A second `impl` block applies only when `T` implements both `PartialOrd` and `Display`:

```rust,ignore
impl<T: PartialOrd + Display> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

A `Pair` of integers has `cmp_display`, because integers meet both bounds. A `Pair` of a type that can't be compared doesn't have the method, and calling it is a
compile error. The sample prints:

```
The largest member is y = 7
```

---

### Blanket implementations

The standard library implements `ToString` for every type that implements `Display`:

```rust,ignore
impl<T: Display> ToString for T {
    // --snip--
}
```

Because of this, any type that implements `Display` gets `to_string`. The sample uses it on an integer:

```rust,ignore
let s = 3.to_string();
println!("{s}");
```

The output is:

```
3
```

You didn't write that implementation. The standard library provides it once, for every type that meets the bound. The documentation for a trait lists its
blanket implementations in the "Implementors" section.

---

### Traits make the checks happen at compile time

Trait bounds let generic code use particular behavior, and the compiler checks every concrete type that's used. A dynamically typed language would report a
missing method at runtime. Rust reports it when the program is compiled, and the code doesn't need runtime checks.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
The largest member is y = 7
3
```

The comments in `src/main.rs` suggest calling `cmp_display` on a `Pair` of a type that doesn't implement `PartialOrd`. The compiler rejects the call, because the
method exists only when the bound is met.

---

*For more detail, see section 10.2 "Defining Shared Behavior with Traits" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html).*
