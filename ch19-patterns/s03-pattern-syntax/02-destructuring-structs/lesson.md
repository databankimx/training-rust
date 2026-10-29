## Destructuring Structs and Using Match Guards

Patterns can take apart structs, enums, tuples, and references. A struct pattern lists the fields it wants, and binds each one to a name. The order doesn't matter, and fields that aren't needed can
be skipped with `..`:

```rust,ignore
let Point { x, y } = p;
```

Here both fields are bound to variables. A pattern such as `Point { x, .. }` binds only `x`, and ignores the rest.

A *match guard* adds an `if` condition to an arm. The arm runs only if its pattern matches and the condition is true. Guards are useful when the pattern alone can't express the rule:

```rust,ignore
match p2 {
    Point { x: x @ 0..=5, y } if x + y != 0 => {
        println!("on the band, x = {x}, y = {y}");
    }
    Point { x, .. } => println!("other, x = {x}"),
}
```

The first arm needs three things. `x` must be in the range 0 to 5, which the `@` binding checks, and `x + y` must not be zero, which the guard checks. The point `(5, -4)` meets both, so the output is:

```
x = 0, y = 7
on the band, x = 5, y = -4
```

The second arm catches every other point. Guards are checked in addition to the pattern, not instead of it, so a guard can't make a pattern match a value it doesn't fit.

---

### Try it in the runner

Press **Run**. The output should match the two lines above.

The comments in `src/main.rs` suggest changing `p2` to `(5, -5)`. The sum of `x` and `y` is zero, so the guard is false, and the second arm runs. The output is `other, x = 5`.

---

*For more detail, see section 19.3 "Pattern Syntax" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html).*
