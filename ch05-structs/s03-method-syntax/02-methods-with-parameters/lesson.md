## Methods with Parameters

Methods can take more parameters after `self`, and those parameters work just like parameters of a function. This step
adds a method that asks whether one rectangle fits inside another.

---

### The can_hold method

The goal is to write this in `main`:

```rust,ignore
println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
```

So `can_hold` is a method that takes a reference to another `Rectangle`, and returns a `bool`. The method is added to the same `impl` block:

```rust,ignore
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

The parameter is `other: &Rectangle`, an immutable borrow. The method only needs to read `other`, not change it, so it borrows it. The caller
writes `&rect2`, and keeps ownership of `rect2` for later use. The return value is a `bool`, which is true when both dimensions of `self` are
larger than those of `other`.

The sample is the full program. Its output is:

```
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

`rect2` is 10 by 40, which fits inside `rect1` (30 by 50) only if both sides are smaller. The width of 10 is smaller, but the height of 40 is smaller too, so it fits.
`rect3` is 60 by 45. Its width is larger than rect1's width of 30, so it can't fit.

Notice that `rect1` takes the place of `self` and is written before the dot. The other rectangle is the first argument after the method name, as in a function call.

---

### Methods can take several parameters

A method can have as many parameters as it needs, and each one has its own type. The rule is always the same: `self` comes first, and the other parameters follow
it, just as they would in a function. The borrow rules from Chapter 4 apply to each one, so a method that only reads a parameter should take a reference to it.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

The comments in `src/main.rs` suggest changing `rect2` to have a width of 30. The result is `false`, because the check is strictly greater than, so a rectangle
with the same width doesn't count as fitting inside.

---

*For more detail, see section 5.3 "Method Syntax" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch05-03-method-syntax.html).*
