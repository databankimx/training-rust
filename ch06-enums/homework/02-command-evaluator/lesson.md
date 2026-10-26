## Homework: Command Evaluator

**The problem:** Model three commands as an enum:

* `Add(i32, i32)`, which adds two numbers.
* `Negate(i32)`, which negates one number.
* `Quit`, which carries no data.

Write `run(cmd: &Command) -> Option<i32>`. It returns `Some` with the result for `Add` and `Negate`, and `None` for `Quit`, to mean the
program should stop.

Then write a `main` that runs `Add(2, 3)`, `Negate(7)`, and `Quit`, in that order. For each one, print `result: ` and the value, or `stopped`
when the result is `None`.

Try writing it yourself before you look at the example.

---

### Hints

* `run` should use a `match` on `cmd`. Each variant needs its own arm, and the compiler will tell you if one is missing.
* Inside a `match` on a reference, the bound names are references. Think about whether `a + b` works on two `&i32` values.
* In `main`, a second `match`, on the result of `run`, decides what to print. Both arms must print something.
* Iterate over `&commands`, so the array is not moved.

---

### Test your code

Your program should print these three lines:

```
result: 5
result: -7
stopped
```

---

### Extensions

* Add a variant `Multiply(i32, i32)`. What does the compiler say about `run`, and what do you need to change?
* Rewrite the second `match` in `main` with `if let`. Which case would you lose the compiler's help with, and why is that a trade-off?
* Make `run` return a `String` instead of an `Option<i32>`, so `Quit` returns `"quit"`. Which of the two designs makes it easier for the caller to stop the loop?

---

*This exercise is not from the book. It practices `match` and enums with data from Chapter 6.*
