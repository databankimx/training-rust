## Solution: Fix the Errors

This walkthrough explains the example solution in `src/main.rs`, which uses the first fix. Read it after you've tried the
exercise yourself.

### Run the solution

Press **F5**. The Output pane shows:

```
best: 30
scores: [10, 20, 30, 40]
```

### Why the original is rejected

The reference `best` borrows `scores`. The `push` needs a mutable borrow of `scores`, which would conflict with the reference
while `best` is still in use. The `push` could also reallocate the vector, which would leave `best` pointing at freed memory.
The program is unsafe, so the rejection is correct.

### The three fixes

**Copy the value out.** `scores[2]` is an `i32`, which is `Copy`, so reading it gives you a value rather than a reference.
No borrow is left over, and the `push` is allowed. This is the sample's fix, and it's the simplest one for numbers.

**Use the reference before the push.** The reference's lifetime ends at its last use, so printing `best` before the `push`
lets the `push` go ahead. The output is the same, but the lines are in a different order in the code.

**Clone the value.** `best.clone()` gives an owned copy. For a number, this costs more than it's worth, because the copy is
already free through `Copy`. For a `String`, a clone copies heap data, which is more expensive, but it is still an option.

The first and second fixes are the idiomatic ones here. The third is useful to know, for cases where the value can't be copied.

---

*This walkthrough accompanies the exercise in Chapter 4 of the course. It's not from the book.*
