## Solution: Median and Mode

This walkthrough explains the example solution in `src/main.rs`. Read it after you've tried the exercise yourself.

### Run the solution

Press **F5**. The Output pane shows:

```
median: 4
mode: 5
```

### How it works

`median` copies the slice into a new vector with `to_vec`, and sorts the copy. The caller's data isn't changed. The middle index is `len / 2`. With an
odd count, that index is the median. With an even count, the function averages the two values on either side of it. Dividing by `2.0` makes the average
a floating-point number, so the result can have a fraction.

`mode` counts each value in a `HashMap`, using `entry` and `or_insert(0)`, as Chapter 8.3 showed. Then `max_by_key` finds the entry with the highest count.
The key for each entry is a pair: the count, and `Reverse(value)`. `Reverse` flips the ordering on values, so among entries with the same count, the
smallest value is the largest key. The tie-break is therefore fixed, and the function returns the same answer every time, even though the hash map's
order varies.

### The extensions

An average needs a floating-point type, because the result can fall between two integers. Dividing two `i32` values in Rust rounds down, so `5 / 2` is 2.

Returning every tied value would make `mode` return a `Vec<i32>`, since there may be more than one.

The mean is the sum divided by the count, which is also a floating-point division. Unlike the median, it doesn't need the list sorted, and one large
value can change it a lot.

---

*This walkthrough accompanies the exercise in Chapter 8 of the course. It's adapted from the summary of Chapter 8 in the book.*
