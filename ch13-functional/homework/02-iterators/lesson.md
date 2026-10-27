## Homework: Iterators

**The problem:** Write your own iterator, and then use iterator adapters on it.

Create a project in VS Code with `cargo new fibonacci_homework`. The Fibonacci sequence starts with 0 and 1, and each later number is the sum of the two before it:
0, 1, 1, 2, 3, 5, 8, 13, and so on.

In `src/main.rs`:

* Define a struct `Fibonacci` that holds the current and next values. Its `new` method starts the sequence at 0 and 1.
* Implement `Iterator` for `Fibonacci`, with `type Item = u64`. Its `next` method returns the current value and moves the sequence on. It never returns `None`,
  so the iterator is infinite.
* In `main`, print the first ten values, using `take`.
* Print the sum of the even Fibonacci numbers below 4,000,000, using `take_while`, `filter`, and `sum`. This is a well-known problem, and the answer is 4613732.

Write tests in a `tests` module. They should check that the first six values are correct, and that the even numbers below 100 add up to 44.

---

### Hints

* Inside `next`, compute the value after the next one before you move the fields on. Otherwise you lose one of the two numbers.
* `take(n)` gives the first `n` items, and `take_while` gives items until the condition first fails.
* The `filter` closure receives a reference, so write `|x| x % 2 == 0`, and the `%` works on `&u64`.

---

### Check your work

Your `main` should print:

```
first ten: [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
even sum below 4000000: 4613732
```

---

### Extensions

* Write the same sum without `take_while`, using a `for` loop. Which version is easier to read, and why?
* Add a method `fibonacci_index(n: u64) -> Option<u64>` that uses the iterator, with `nth`, to return the `n`th number. Why is `Option` the right return type?

---

*This exercise is not from the book. It practices iterators from Chapter 13.*
