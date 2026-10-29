## Using Box<T> to Store Data on the Heap

The simplest smart pointer is `Box<T>`. A box stores its value on the heap, rather than on the stack. What stays on the stack is the pointer to the heap data. Boxes
have no performance overhead beyond that, and they have few extra features. A box is most useful in three situations:

* When a type's size can't be known at compile time, and a value of it is needed where an exact size is required.
* When there's a large amount of data, and ownership should move without copying the data.
* When you own a value, and only care that it implements a particular trait. Chapter 18 covers this case, which is called a trait object.

This step stores a single value in a box:

```rust,ignore
let b = Box::new(5);
println!("b = {b}");
```

The output is:

```
b = 5
```

A box can be used much like the value inside it. Here `println!` formats `b`, and the value is shown. When `b` goes out of scope at the end of `main`, both the box on the
stack and the value on the heap are freed.

A single value on the heap isn't useful in practice, and the stack is the right place for most values. The next step shows a case where a box makes a type possible at all.

---

### Try it in the runner

Press **Run**. The output should be `b = 5`.

The comments in `src/main.rs` suggest changing the argument `b` to `*b`. The `*` follows the pointer to the value it holds, so the output is the same. The
next step uses `*` in a different way.

---

*For more detail, see section 15.1 "Using Box<T> to Point to Data on the Heap" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch15-01-box.html).*
