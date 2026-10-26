## Solution: Optional Port

This walkthrough explains the example solution in `src/main.rs`. Read it after you've tried the exercise yourself.

### Run the solution

Press **F5**. The Output pane shows:

```
listening on port 3000
no port configured, using 8080
```

### How it works

The `let...else` in `describe` matches `config.port` against `Some(port)`. If the pattern matches, `port` is bound and the function continues. If it
doesn't, the `else` block runs, and it returns the default message. The `else` must leave the function, which the `return` does.

`config.port` is an `Option<u16>`, and `u16` is `Copy`, so binding `port` copies the number out of the struct. The struct isn't moved, and `describe` only
borrows its `Config`.

After the `let...else`, the rest of the function works with `port` as a plain `u16`. There's no second branch to follow, so the main path reads from
top to bottom.

### The extensions

An `if let` version works too, but it nests the message inside the block, and the `else` branch returns the default. Both versions are correct. The
`let...else` version reads better when the missing case is an early exit, and the rest of the function is the real work.

Adding a `host` field follows the same pattern: a second `let...else` line, with its own early return. Each missing value becomes one early exit, and the
function's main path stays flat.

Taking `Config` by value would give the caller up the struct, because `describe` would own it. That's acceptable when the caller doesn't need the config
afterwards, but it's a more restrictive API, as the ownership inventory showed.

---

*This walkthrough accompanies the exercise in Chapter 6 of the course. It's not from the book.*
