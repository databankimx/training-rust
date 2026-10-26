## Solution: Command Evaluator

This walkthrough explains the example solution in `src/main.rs`. Read it after you've tried the exercise yourself.

### Run the solution

Press **F5**. The Output pane shows:

```
result: 5
result: -7
stopped
```

### How it works

`run` takes a reference to the command, so the array in `main` isn't moved. Inside the `match`, `a`, `b`, and `n` are references to the numbers, because
the match is on a reference. Rust allows `a + b` on two `&i32` values, and `-n` on a `&i32`, so the arithmetic works without any dereferencing.

Each variant has its own arm. `Quit` has no data, so its arm just returns `None`. The compiler checks that all three variants are covered.

In `main`, the loop goes over `&commands`, which borrows the array. The inner `match` handles both cases of the `Option`: `Some` prints the value, and `None`
prints `stopped`. Both arms are needed, because a `match` must cover every case.

### The extensions

Adding `Multiply` makes `run` fail to compile, because its `match` is no longer exhaustive. The compiler names the missing pattern. You add an arm for it,
and `main` doesn't need to change, because the `Option` return type already fits.

Using `if let Some(value) = run(cmd)` in `main` would print values, but nothing would be printed for `None`. You'd lose the compiler's check that both cases
are handled. For a program that only prints results, that loss may be acceptable, but it's a choice to make deliberately.

Returning a `String` lets `Quit` return a message, but the caller then has to compare strings to find out whether to stop. `Option<i32>` says exactly
which case means stop, and the type system enforces it.

---

*This walkthrough accompanies the exercise in Chapter 6 of the course. It's not from the book.*
