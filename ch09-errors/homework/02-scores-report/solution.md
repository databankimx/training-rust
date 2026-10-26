## Solution: Scores Report

This walkthrough explains the example solution in `src/main.rs`. Read it after you've tried the exercise yourself.

### Run the solution

Press **F5**. The Output pane shows:

```
Ok("mean 9.0 is 90.0% of 10")
Err("cannot average an empty list")
Err("whole is zero")
```

### How it works

Each function checks its own failure case first, and returns an `Err` with a message when it happens. The success case comes last, wrapped in `Ok`.

`score_report` is a chain. `average(scores)?` returns the mean, or hands the error from `average` to the caller. The same is true of `percent_of`. If either
step fails, the function returns straight away, and the `format!` is never reached. That's the reason `?` is used here: each step depends on the one before.

The division in `average` uses `as f64` on the length, because `len` returns a `usize`, and dividing a float by an integer type is a compile error. The
conversion is explicit, so the reader can see where the number changes type.

`main` prints each result with `{:?}`. That shows `Ok` or `Err`, along with the text, so the three cases are easy to tell apart. A real program would match
on the result and print the message in a friendlier form.

### The extensions

`score_report` returns the error to its caller, so `main` decides what to print. A function that shows errors to the user is usually the best place to decide, because
it knows the context, and the lower-level functions don't.

An empty list is an error for the median for the same reason as for the mean. There is no middle value of nothing. A function that needs a value to exist can say so
with its return type, which is what `Result` does.

A custom error enum, with one variant for an empty list and one for a zero whole, lets a caller match on the kind of failure, rather than comparing text. The
trade-off is more code, which is worth it when callers need to react differently to each kind of error.

---

*This walkthrough accompanies the exercise in Chapter 9 of the course. It's not from the book.*
