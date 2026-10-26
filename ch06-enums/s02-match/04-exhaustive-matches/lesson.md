## Exhaustive Matches and Catch-All Patterns

Every `match` must cover every possible value. The compiler checks this, and refuses to compile a `match` that leaves a case out.
This is one of the most useful things about `match`, and it applies to every enum.

---

### Matches must be exhaustive

Here's a version of `plus_one` that forgets the `None` case:

```rust,ignore
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

The compiler rejects it, with an error like this:

```text
error[E0004]: non-exhaustive patterns: `None` not covered
```

The compiler names the pattern you missed. The match must *exhaust* every possibility, and here `None` is left over. This is how Rust
prevents the bug of assuming a value is present when it might be absent.

---

### Catch-all patterns

Sometimes most values should do one thing, and a few special values should do something else. A catch-all arm handles every value that
the earlier arms don't match. The sample is a dice game: a roll of 3 or 7 is special, and every other roll moves the player:

```rust,ignore
let action = match roll {
    3 => String::from("gain a fancy hat"),
    7 => String::from("lose a fancy hat"),
    other => move_player(other),
};
```

The last arm's pattern is a name, `other`. It matches any value that the earlier arms didn't, and binds it to `other`, so the arm can use it.

The sample's output is:

```
roll 3: gain a fancy hat
roll 7: lose a fancy hat
roll 9: move 9 spaces
```

The catch-all arm must come last. Arms are tried in order, so an arm placed before the catch-all would match first, and the catch-all would
only see values that the earlier arms missed. The compiler warns you if you add an arm after a catch-all, because that arm could never run.

---

### The underscore placeholder

If the catch-all doesn't need the value, use `_` instead of a name. The pattern `_` matches any value and doesn't bind it:

```rust,ignore
let action = match roll {
    3 => String::from("gain a fancy hat"),
    7 => String::from("lose a fancy hat"),
    _ => String::from("roll again"),
};
```

Now the rules say that any other roll means you roll again, and the value isn't needed. The `_` tells Rust that the value is deliberately ignored,
so it doesn't warn about an unused variable.

If nothing should happen for the other values, the arm's code can be the unit value `()`, which means nothing. The match is still exhaustive,
because the `_` arm covers everything else.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
roll 3: gain a fancy hat
roll 7: lose a fancy hat
roll 9: move 9 spaces
```

The comments in `src/main.rs` suggest replacing the catch-all with `_`, and returning `"roll again"`. The roll of 9 now prints `roll 9: roll again`,
because the value isn't bound, and the arm doesn't move the player.

---

*For more detail, see section 6.2 "The match Control Flow Construct" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch06-02-match.html).*
