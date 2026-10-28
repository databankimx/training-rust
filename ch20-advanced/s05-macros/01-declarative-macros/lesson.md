## Declarative Macros

A macro writes code for you. Rust has two kinds: declarative macros, written with `macro_rules!`, and procedural macros. This step covers declarative macros, which work like a `match` on the source code
of a call. Each arm has a pattern, and when the call matches it, the code in the arm replaces the call. That happens before the compiler checks types, so a macro can accept things a function can't.

The sample is a simplified version of `vec!`:

```rust,ignore
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

The pattern `$( $x:expr ),*` means: match zero or more expressions, separated by commas. The name `$x` is bound to each one. The `$( ... )*` in the body repeats the code once for each match. A call such as `my_vec![1, 2, 3]` expands to:

```rust,ignore
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

A function can't do this, because a function's parameters have a fixed number and type, and `my_vec` takes any number of any type. The output of the sample is:

```
numbers: [1, 2, 3]
words: ["a", "b"]
empty: []
```

The last call has no values, so the repetition runs zero times. The type comes from the annotation, `Vec<i32>`.

**Macros are checked after they expand.** The expanded code is ordinary Rust, so mistakes in the body show up as errors in the expansion, and the error messages can be harder to read than those from functions.

---

### Try it in the runner

Press **Run**. The output should match the three lines above.

The comments in `src/main.rs` suggest calling `my_vec![1 + 1, 2 * 3]`. Each `$x:expr` captures a whole expression, so the call pushes `2` and `6`, and the output is `[2, 6]`.

---

*For more detail, see section 20.5 "Macros" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch20-05-macros.html).*
