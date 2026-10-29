## Generic Lifetimes in Functions

A function that takes references and returns one needs a way to say how the returned reference is related to the inputs. Rust answers with generic lifetime
parameters, declared in angle brackets after the function name.

---

### The problem

Suppose you want a function that returns the longer of two string slices. Without annotations, the compiler can't tell whether the result refers to `x` or to
`y`, because the `if` returns one and the `else` returns the other. It also doesn't know how long each input lives. So it refuses, and asks for a lifetime.

---

### The annotation

The signature declares one lifetime, `'a`, and gives it to all three references:

```rust,ignore
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
```

The signature says this: for some lifetime `'a`, both inputs live at least that long, and the returned reference lives at least that long too. In practice, the
result lives no longer than the shorter of the two inputs. The annotation doesn't change how long anything lives. It tells the borrow checker what to check.

The lifetime is part of the function's contract, like its types. The annotations go in the signature, not the body. The function itself doesn't need to know how
long the inputs will live, only that some lifetime satisfies the signature.

---

### The sample

The sample passes two `String`s, one of which lives in an inner block. The result is used only inside that block, so the borrow checker accepts it:

```
The longest string is long string is long
```

The result is `string1`, which is longer, and it's used while `string2` is still alive.

---

### Why the lifetime matters

If you move the `println!` after the inner block, `result` might refer to `string2`, which has been freed by then. The borrow checker rejects the program, because
the signature promised that the result lives as long as the shorter input. The compiler can't see which input `longest` returned at runtime, so it assumes the shorter
lifetime.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
The longest string is long string is long
```

The comments in `src/main.rs` suggest moving the `println!` out of the inner block. Rust reports that `string2` does not live long enough, for the reason above.

---

*For more detail, see section 10.3 "Validating References with Lifetimes" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html).*
