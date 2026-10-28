## Pattern Syntax: Literals, Ranges, and Bindings

Patterns have their own syntax, which covers the ways a value can be compared and the ways names can be bound. This step covers the ones used most often.

**Literals.** A pattern can be a literal value, such as `1`. It matches only that value, and the match is checked when the program runs.

**Multiple patterns.** `|` matches any of several patterns, so `2 | 3` matches either value.

**Ranges.** `..=` matches an inclusive range, so `4..=9` matches 4, 5, and every value up to 9. Ranges work only with numbers and `char` values, because the compiler must be able to check that a range is valid.

**The wildcard.** `_` matches anything, without binding it to a name. It's often used as the last arm, to catch the cases the other arms didn't cover.

**Binding with `@`.** `id @ 40..=49` tests the range, and binds the matched value to the name `id` at the same time. Without `@`, a range pattern can only test the value, not keep it.

```rust,ignore
let message = match 42 {
    id @ 40..=49 => format!("id {id} is in the forties"),
    _ => String::from("other"),
};
```

The output of the sample is:

```
1: one
3: two or three
7: four to nine
12: something else
id 42 is in the forties
```

The arms are checked in order, and the first one that matches is used. The `_` arm is last, so it only runs when nothing else matches.

---

### Try it in the runner

Press **Run**. The output should match the five lines above.

The comments in `src/main.rs` suggest changing 42 to 52. No range matches, so the `_` arm runs, and the message is `other`.

---

*For more detail, see section 19.3 "Pattern Syntax" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html).*
