## Refactoring with Tuples

The first version passed two separate values, which hid the fact that they belong together. A tuple is the obvious next step:
it groups the two values into one argument.

---

### The tuple version

Here the width and height are one value, a tuple, passed to `area`:

```rust,ignore
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

This is the sample for this step. Its output is the same as before:

```
The area of the rectangle is 1500 square pixels.
```

The function now takes one argument, which is an improvement. It also says the two values belong together.

---

### What the tuple still hides

A tuple doesn't name its elements. Inside `area`, the width is the tuple index `0` and the height is the index `1`. The calculation
`dimensions.0 * dimensions.1` doesn't say which is which, and the reader has to remember the convention.

For an area calculation that doesn't matter, since multiplication is the same either way. But suppose you draw the rectangle on the
screen. Mixing up the width and the height would then give you a tall, thin rectangle instead of a wide, short one, and the code would
still compile and run. The type `(u32, u32)` says nothing about which number is which.

So the tuple adds some structure, but not enough meaning. The next step uses a struct, which can name each part.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
The area of the rectangle is 1500 square pixels.
```

The comments in `src/main.rs` suggest swapping the tuple's values. The program still compiles and prints the same area, which shows that
the code doesn't record which value is the width.

---

*For more detail, see section 5.2 "An Example Program Using Structs" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch05-02-example-structs.html).*
