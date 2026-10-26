## Other Slices

String slices are specific to text, but Rust has a general slice type for any sequence. Just as you might want part of
a string, you might want part of an array.

---

### An array slice

```rust,ignore
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

The slice `&a[1..3]` covers the elements at indexes 1 and 2, so it holds `2` and `3`. The end index is not included,
just as with string slices.

The type of this slice is `&[i32]`. It works in the same way as a string slice: it stores a reference to the first
element, and a length. You'll use this kind of slice with many collections. Vectors, which you'll meet in Chapter 8, use the
same idea.

The sample for this step prints the slice:

```
[2, 3]
```

---

### The same idea, for any sequence

The parts of a slice are always the same:

* A pointer to the first element in the range.
* A length, which tells the slice how many elements it covers.

That's why a slice is a fat pointer, just as a string slice is. The compiler checks that the slice stays inside the
collection it was taken from, and that the collection isn't changed while the slice is in use.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
[2, 3]
```

The comments in `src/main.rs` suggest two other ranges to try: `..2`, which starts at the beginning, and `3..`, which runs to
the end. Predict the output before you run each one.

---

*For more detail, see section 4.4 "The Slice Type" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch04-04-slices.html).*
