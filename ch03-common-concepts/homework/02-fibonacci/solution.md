## Solution: Fibonacci Numbers

This walkthrough explains the example solution in `src/main.rs`. Read it after you've tried the
exercise yourself. Your version may be quite different, and that's fine.

### Run the solution

Press **F5** and enter `10`. The Output pane shows:

```
Which Fibonacci number do you want? Enter a whole number:
10
The Fibonacci number at position 10 is 55.
```

Try `0`, which gives `0`, and `1`, which gives `1`. Then try a big number, such as `100`. The program
doesn't crash. It tells you the number is too big.

### How it works

The heart of the solution is the loop:

```rust
let mut previous: u64 = 0;
let mut current: u64 = 1;

for _ in 1..n {
    let next = previous.checked_add(current)?;
    previous = current;
    current = next;
}
```

This fragment is part of the sample program, and prints nothing on its own. Each time round the loop, `next` is the sum of the two numbers we have. Then `previous` moves on to
`current`, and `current` moves on to `next`. After `n - 1` steps, `current` holds the nth number.

A few things here are worth noticing:

* `for _ in 1..n` repeats the loop without needing the counter's value, so we use `_`.
* `checked_add` returns `None` if the addition would overflow, rather than wrapping around or panicking.
  That's the method you met in the scalar types step.
* The `?` after `checked_add` means "if this is `None`, return `None` from the whole function". Because
  `fibonacci` returns an `Option`, this is the right way to stop early.

The loop computes the nth number as its last step, so it never works out a number it doesn't need. That
matters for the size limit. A `u64` holds numbers up to about 18 quintillion, and the 93rd Fibonacci
number is the largest one that fits. For `n` above 93, the function returns `None`, and `main` prints the
message.

The `if n == 0` at the top handles the one case the loop doesn't cover. With no steps, the answer is simply 0.

---

*This walkthrough accompanies the exercise in [The Rust Programming Language](https://rust-book.cs.brown.edu/ch03-05-control-flow.html#summary).*
