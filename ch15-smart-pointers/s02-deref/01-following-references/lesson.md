## Following References and Boxes with *

The dereference operator, `*`, follows a pointer to the value it points to. It's the same operator as the one used for multiplication and in glob imports, but in this
role it means "the value behind this pointer".

A reference is a kind of pointer, and it's the simplest one to follow:

```rust,ignore
let x = 5;
let y = &x;
println!("reference: {}", *y);
```

`y` holds the address of `x`, and `*y` gives the `i32` stored there. The output is:

```
reference: 5
```

The `*` is needed when you compare or calculate with the value. Writing `assert_eq!(5, y)` compares a number with a reference, and that isn't allowed, because the two
are different types. You must follow the reference with `*` first.

A `Box` works in the same way:

```rust,ignore
let b = Box::new(x);
println!("box: {}", *b);
```

The box holds a copy of `x` on the heap, and `*b` follows its pointer to that value. The output is:

```
box: 5
```

Both `*y` and `*b` produce the value, and the code that uses them can be the same. The next step shows how a type of your own can behave the same way.

---

### Try it in the runner

Press **Run**. The output should be `reference: 5` and `box: 5`.

The comments in `src/main.rs` suggest adding `assert_eq!(5, y);` without the `*`. The compiler rejects it, because `5` is an `i32` and `y` is a reference to one.

---

*For more detail, see section 15.2 "Treating Smart Pointers Like Regular References with the Deref Trait" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch15-02-deref.html).*
