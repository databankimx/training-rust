## Homework: Fix the Errors

**The problem:** The program below is rejected by the borrow checker. Fix it, and then fix it in two other ways.

```rust,ignore
fn main() {
    let mut scores = vec![10, 20, 30];
    let best = &scores[2];
    scores.push(40);
    println!("best: {best}");
    println!("scores: {:?}", scores);
}
```

The program should print the best score, and then the scores, including the 40 added after the reference was made.

Try each fix yourself before you look at the example.

---

### Hints

* Ask first: is the program unsafe, or is the borrow checker being too strict? Here, the program is unsafe. `push` could reallocate
  the vector, and `best` would point at freed memory.
* The first fix is to stop holding a reference across the `push`. What do you need from `scores[2]`, and can you get it as a value?
* The second fix is to use the reference before the `push`, so that its lifetime ends first. What has to change in the order of the lines?
* The third fix is to clone the value. Think about whether that's worth the cost for a number, and what it would cost for a `String`.

---

### Test your fixes

Each of your three versions should print the same two lines:

```
best: 30
scores: [10, 20, 30, 40]
```

For the second fix, the line that prints `best` must come before the `push`, and the output would then be `best: 30` followed by the
scores. Check that your version prints in the order you expect.

---

### Extensions

* Change the vector to hold `String` values, and try the same three fixes. Which one still works without a change in meaning?
* Explain, in one sentence each, why the borrow checker rejects the original program.

---

*This exercise is not from the book. It practices the fixes from Chapter 4's fixing ownership errors step.*
