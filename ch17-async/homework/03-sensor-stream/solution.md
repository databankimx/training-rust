## Solution: A Sensor Stream

This walkthrough explains the example solution in `src/main.rs`. Read it after you've written your own version.

### Run the solution

Press **Run** for the output, and **Test** for the two tests. The output is:

```
accepted 12
accepted 15
accepted 14
accepted 13
total: 54
```

### How it works

`total_of_valid` builds a stream from the readings, keeps those in the range 0 to 100, and takes the first four. Each step is a method on the stream, so the chain reads from top to bottom.

The loop pulls one reading at a time with `next().await`. Items are produced only when they're asked for. Once `take` has delivered four readings, it reports the end of the stream, and the loop stops. The
later readings are never examined, which is why 16 isn't printed.

The function is `async`, because it uses `await`. The `main` function is not, so it runs the function with `trpl::block_on`. The tests do the same, and check the total for the example list and for a list
with no valid readings. The total for the empty case is zero, because the loop never runs.

### The extensions

Keeping readings below zero, as well as the valid range, changes the filter to something like `reading < 0 || (0..=100).contains(reading)`. The accepted values change, and so does the total.

An average needs a sum and a count. Dividing two `i32` values truncates the result to a whole number, so an average such as 13.5 would print as 13. A `f64` keeps the fraction. Convert each
value with `as f64` before dividing, and print with `{:.1}`.

---

*This walkthrough accompanies the exercise in Chapter 17 of the course. It's not from the book.*
