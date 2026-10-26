## Homework: Median and Mode

**The problem:** Given a list of integers, find:

* The **median**: the middle value once the list is sorted. If the list has an even number of values, the median is the average of the two middle values.
* The **mode**: the value that occurs most often. A hash map is a natural way to count occurrences.

Write `median(values: &[i32]) -> Option<f64>` and `mode(values: &[i32]) -> Option<i32>`. Return `None` for an empty list. Then use them in `main` on the
list `3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5`.

Try writing it yourself before you look at the example.

---

### Hints

* To find the median, sort a copy of the list. Sorting the original would change the caller's data, which the function shouldn't do.
* With an odd count, the middle index is `len / 2`. With an even count, you need the two values around that index.
* To count, use `entry(value).or_insert(0)`, and add one to the count, as in Chapter 8.3.
* A mode can tie. Decide what your function returns when two values share the highest count, and make it do that every time.

---

### Test your code

Your program should print these two lines:

```
median: 4
mode: 5
```

Check the answers by hand. Sorted, the list is `1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9`. It has 11 values, so the median is the sixth one, which is 4.
The value 5 appears three times, more than any other.

---

### Extensions

* Return the median as an `f64` even when the list has an odd length. Why does an average need a floating-point type?
* Change `mode` to return every value that ties for the highest count. What type should it return?
* Find the mean as well, and explain why it needs a different type from the median.

---

*This exercise is adapted from the summary of Chapter 8 in [The Rust Programming Language](https://doc.rust-lang.org/book/ch08-03-hash-maps.html).*
