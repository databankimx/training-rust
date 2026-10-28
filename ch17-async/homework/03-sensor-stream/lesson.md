## Homework: A Sensor Stream

**The problem:** A sensor produces readings as a stream. Some readings are invalid, because they're outside the range 0 to 100. Keep the first four valid readings, print each one, and
print their total.

Create a project in VS Code with `cargo new sensor_stream`, then add the crate with `cargo add trpl@0.3.0`. In `src/main.rs`:

* Make a stream of readings from the array `[12, -1, 15, 300, 14, 13, -5, 16]`, using `trpl::stream_from_iter`.
* Use `filter` to keep only the readings from 0 to 100 inclusive, and `take` to stop after four.
* Loop over the stream with `while let`, using `next().await`, and print each reading as `accepted <value>`. Add each one to a running total.
* After the loop, print the total as `total: <sum>`.
* Run everything inside `trpl::block_on`.

Write a test in a `tests` module that calls a function `total_of_valid(readings: Vec<i32>) -> i32`, which does the same work, and returns the total. Check the total for the array above, and
for a list with no valid readings.

---

### Hints

* Import `trpl::StreamExt`, or the `next` method will not be found.
* `(0..=100).contains(&value)` checks a range. The `filter` closure receives a reference, so compare with `reading` directly.
* Put the work in `total_of_valid` as an `async fn`, so the test and `main` can both use it.

---

### Check your work

Your program should print:

```
accepted 12
accepted 15
accepted 14
accepted 13
total: 54
```

The readings 300, -1, and -5 are dropped by the filter, and 16 is never reached, because `take` has already produced four values.

---

### Extensions

* Change the filter to keep readings below 0 as well, and see how the output changes.
* Compute the average of the accepted readings, and print it with one decimal place. What type should you use, and why does `i32` not work?

---

*This exercise is not from the book. It practices streams, `filter`, and `take` from Chapter 17.*
