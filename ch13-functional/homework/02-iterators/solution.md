## Solution: Iterators

This walkthrough explains the example solution in `src/main.rs`. Read it after you've written your own version.

### Run the solution

Press **Run** for the output, and **Test** for the tests. The output is:

```
first ten: [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
even sum below 4000000: 4613732
```

### How it works

`Fibonacci` implements `Iterator` by defining one method, `next`, and an associated type, `Item`. Once that's done, every adapter in the standard library works on it,
including `take`, `take_while`, `filter`, and `sum`. That's the reason to implement the trait, rather than writing a function that returns a `Vec`.

`next` computes the following value first, then moves the fields on. If it set `self.current` before saving the old value, the old value would be lost, and the sequence
would stop being Fibonacci.

The iterator is infinite, so it never returns `None`. The adapters decide when to stop. `take(10)` stops after ten values, and `take_while` stops at the first value
that is 4,000,000 or more. Because `take_while` checks each value before passing it on, the sequence is never computed much past the limit.

`filter(|x| x % 2 == 0)` receives a reference, `&u64`. The `%` operator is implemented for references to integers, so the expression works without a `*`.

### The extensions

A `for` loop version reads more like the steps being taken: a `let mut total` and an `if` inside a `loop` that breaks at the limit. The iterator version is shorter, and
states the rule directly. Which is easier to read is a matter of taste, but the iterator version is the one that composes well with other adapters.

`nth` consumes items until it reaches the requested position, and returns `Option`, because the position may not exist in a finite sequence. Here the sequence is infinite,
so the result is always `Some`. The `Option` is still the right return type, because a future iterator might be finite, and the caller should have to handle that case.

---

*This walkthrough accompanies the exercise in Chapter 13 of the course. It's not from the book.*
