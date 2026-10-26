## Separate Variables

Suppose you want a program that calculates the area of a rectangle, given its width and height in pixels. That's a good
example for seeing when structs help, because the first version works, but its structure hides what the values mean.

---

### The first version

The simplest program passes the width and height to a function separately:

```rust,ignore
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

This is the sample for this step. Its output is:

```
The area of the rectangle is 1500 square pixels.
```

It works, but look at the signature of `area`:

```rust,ignore
fn area(width: u32, height: u32) -> u32 {
```

`area` is supposed to calculate the area of one rectangle, yet it takes two separate parameters, and nothing in the program says
they belong together. A rectangle's width and height are a pair. Keeping them apart means the program depends on remembering that
connection.

---

### Why grouping matters

Keeping related values together makes the code easier to read and harder to get wrong. If a second rectangle appeared, you'd have to
keep four separate numbers in step, and nothing would stop you passing the width of one and the height of another.

The next step shows the first attempt to group them, with a tuple. The step after that uses a struct, which gives the group a name.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
The area of the rectangle is 1500 square pixels.
```

The comments in `src/main.rs` suggest swapping the arguments in the call to `area`. The program still compiles, and the answer is the same,
because multiplication doesn't depend on the order. That's a hint about the problem: nothing in the code says which number is the width.

---

*For more detail, see section 5.2 "An Example Program Using Structs" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch05-02-example-structs.html).*
