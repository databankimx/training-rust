## let...else

Many functions do some work when a value is present, and return early when it isn't. `if let` can do this, but it pushes the work
into a nested block, and the two branches of the function start to look alike. `let...else` keeps the main work at the top level.

---

### The early-return problem

The sample describes a state quarter. A penny isn't a quarter, so the function should stop there and return `None`. With `if let`, the
work would sit inside the block, and the `None` case would be the `else`:

```rust,ignore
fn describe_state_quarter(coin: &Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        // all the work happens inside the block
    } else {
        return None;
    }
}
```

This works, but the function has two branches that both matter, and the main work is hidden inside one of them.

---

### let...else

`let...else` takes a pattern on the left and an expression on the right, like `if let`. It has no `if` branch. If the pattern matches, the names it
binds are available for the rest of the function. If it doesn't match, the `else` block runs, and that block must leave the function, usually with
`return`:

```rust,ignore
fn describe_state_quarter(coin: &Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
```

After the `let...else`, the function reads as one straight path. The `state` name is available below the statement, and the early return is the only
thing that happens for a non-quarter. The `else` block has to diverge, so the compiler won't let it fall through.

---

### The sample

The sample calls the function for two quarters and a penny:

```
Some("Alabama is pretty old, for America!")
Some("Alaska is relatively new.")
None
```

The `existed_in` method compares the year against when each state joined the Union. Alabama joined in 1819, so it existed in 1900. Alaska joined in
1959, so it didn't. The penny has no state, so the function returns `None` straight away.

---

### Choosing between the three

Use `match` when every case needs its own result. Use `if let` when only one case does something, and the rest can be ignored. Use `let...else` when
one case is required for the rest of the function to make sense, and the other case should stop the function early. The book describes this as staying on the "happy path".

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Some("Alabama is pretty old, for America!")
Some("Alaska is relatively new.")
None
```

The comments in `src/main.rs` suggest changing the year to 1960. Alaska's description then changes to "pretty old", because Alaska existed in 1960.
Alabama's description doesn't change.

---

*For more detail, see section 6.3 "Concise Control Flow with if let and let...else" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch06-03-if-let.html).*
