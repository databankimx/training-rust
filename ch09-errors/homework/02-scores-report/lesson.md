## Homework: Scores Report

**The problem:** Build a short report from a list of scores. The report needs the mean of the scores, and the mean as a percentage of a total. Each
step can fail, and each one returns a `Result`:

* `average(values: &[f64]) -> Result<f64, String>` returns the mean. An empty list is an error.
* `percent_of(part: f64, whole: f64) -> Result<f64, String>` returns the part as a percentage of the whole. A zero whole is an error.
* `score_report(scores: &[f64], total: f64) -> Result<String, String>` combines them into a sentence, using `?` to pass any error up.

The sample calls `score_report` three times: with valid scores, with an empty list, and with a zero total.

Try writing it yourself before you look at the example.

---

### Hints

* Each function needs an early `return Err(...)` for its failure case. Then the success case is the last expression.
* In `score_report`, `average(scores)?` gives the mean, or returns the error from `average`. The same works for `percent_of`.
* `format!` with `{mean:.1}` prints a number with one decimal place.
* `{:?}` in `main` shows the whole `Result`, including `Ok` or `Err`, which makes the three cases easy to tell apart.

---

### Test your code

Your program should print these three lines:

```
Ok("mean 9.0 is 90.0% of 10")
Err("cannot average an empty list")
Err("whole is zero")
```

Check the first line by hand. The mean of 8, 9, and 10 is 9. Nine out of a total of 10 is 90 percent.

---

### Extensions

* Make `score_report` print the error, rather than returning it, when the scores are empty. Which function should decide what to do with the error?
* Add a `median` function that returns a `Result`. Should an empty list be an error, the same as for the mean? Why, or why not?
* Replace `String` as the error type with your own enum, with a variant for each kind of error. What does the caller gain?

---

*This exercise is not from the book. It practices `Result` and `?` from Chapter 9.*
