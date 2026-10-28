## The Never Type

Rust has a type written `!`, called the *never type*. It has no values, and it's the return type of a function that never returns. A function such as `fn fail() -> !` can only end by panicking, looping forever, or
exiting the program.

The never type matters because `match` arms must all produce the same type. The sample has two arms that don't produce a value at all:

```rust,ignore
let n: u32 = match input.parse() {
    Ok(n) => n,
    Err(_) => continue,
};
```

`continue` jumps back to the top of the loop, so it never produces a `u32`. Its type is `!`, and a `!` can be used where any type is expected. The same applies to a call to a function that returns `!`:

```rust,ignore
let value: i32 = match Some(3) {
    Some(v) => v,
    None => fail("missing"),
};
```

`fail` has type `!`, so the `None` arm fits the `i32` the other arm produces. This is also how `unwrap` works: its `None` arm panics, so no value is needed there.

The sample adds the numbers that parse, skipping the bad one, and prints the total. Then it takes the `Some` path, so `fail` never runs:

```
total: 19
value: 3
```

The total is 7 + 12. The `"x"` input fails to parse, so `continue` skips it.

---

### Try it in the runner

Press **Run**. The output should match the two lines above.

The comments in `src/main.rs` suggest changing `Some(3)` to `None`. The program prints `total: 19`, and then `fail` panics with `fatal: missing`, which ends the program.

---

*For more detail, see section 20.3 "Advanced Types" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch20-03-advanced-types.html).*
