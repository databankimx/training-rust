## Example: Applying a Curve

This is the second program of the Ownership Inventory #3. A test result holds the scores and an optional curve, and `apply_curve` adds the curve to every
score. The program looks like a simple use of a helper method, and it runs into a limitation of the borrow checker.

---

### The program

Here is the method as it first appears. It asks a helper, `get_curve`, for the curve, and loops over the scores:

```rust,ignore
pub fn apply_curve(&mut self) {
    if let Some(curve) = self.get_curve() {
        for score in self.scores.iter_mut() {
            *score += *curve;
        }
    }
}
```

The compiler rejects it:

```text
error[E0502]: cannot borrow `self.scores` as mutable because it is also borrowed as immutable
```

The reason is lifetime elision. `get_curve` takes `&self` and returns a reference, so the output borrows the whole of `self`, not just `self.curve`. The `curve`
reference is alive for the whole `if let` block, so `self` stays borrowed, and `self.scores.iter_mut()` can't borrow `scores` mutably.

---

### Is the original safe?

Yes, as written. The borrow checker is being cautious. The helper borrows all of `self`, but it only reads `curve`, which doesn't overlap `scores`. The inventory's
context says that if `get_curve` ever returned a reference into `scores`, the program would be a real problem, but as it stands none of the candidate programs
causes undefined behavior.

---

### The fix

`Option<usize>` is `Copy`, so the function can copy the curve out of the struct, and then no borrow of `self` is held during the loop:

```rust,ignore
pub fn apply_curve(&mut self) {
    if let Some(curve) = self.curve {
        for score in self.scores.iter_mut() {
            *score += curve;
        }
    }
}
```

The sample's output is:

```
[30, 60, 40]
```

Each score went up by 10. The curve is read first, so only `scores` is borrowed mutably during the loop, and the struct is changed exactly as the code intends.

---

### Why the other candidates are weaker

* Calling `.as_ref()` on the helper's result still borrows `self`, so the conflict stays.
* Cloning `scores` makes the loop change a copy, so the caller's scores are never updated. That breaks the intent.
* Using `iter()` instead of `iter_mut()` reads the scores, but it can't change them, so the loop no longer does its job.

The fix from the inventory is the one that copies the `Option`. The book also notes `Option::copied`, which does the same job on the helper's result, and it releases
the borrow as soon as it's called.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
[30, 60, 40]
```

The comments in `src/main.rs` suggest setting `curve` to `None`, and calling `apply_curve` again. The scores then don't change, because there's no curve to add.

---

### Quiz

*These questions are adapted from the Ownership Inventory in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Consider the original `apply_curve` method above. If you tried to compile it, which of the following best describes the compiler error you would get?

○ in `get_curve`, cannot return a reference to a local variable `self.curve`
→ Not quite. `get_curve` returns a reference to a field of `self`, which is allowed, because `self` outlives the call.
○ in `apply_curve`, cannot borrow `self` as immutable for `get_curve`
→ Not quite. Borrowing `self` immutably for `get_curve` is allowed on its own. The conflict happens later, in the loop.
○ in `apply_curve`, `*score` cannot be mutated
→ Not quite. `*score` can be mutated, because `iter_mut` gives mutable references to each score.
◉ in `apply_curve`, cannot borrow `self.scores` as mutable for `iter_mut`
→ Yes. The borrow from `get_curve` still covers all of `self`, so `self.scores` can't be borrowed mutably.
````

````quiz
Consider the original `apply_curve` method above. Assume that the compiler did not reject it. Which of the following programs would (1) pass the compiler, and
(2) possibly cause undefined behavior if executed?

○ `let mut result = TestResult { scores: vec![20, 50, 30], curve: Some(10) }; result.apply_curve();`
→ Not quite. This program is safe. It changes the scores in place, and nothing else refers to them.
○ `let mut result = TestResult { scores: vec![20, 50, 30], curve: Some(10) }; let x = &result.scores[0]; result.apply_curve(); println!("{}", x);`
→ Not quite. The `apply_curve` call changes the scores while `x` refers to one of them, but `x` isn't freed, and the program is memory safe, since the vector's
buffer doesn't move.
○ `let mut result = TestResult { scores: vec![20, 50, 30], curve: Some(10) }; result.apply_curve(); println!("{:?}", result.scores);`
→ Not quite. This program prints the scores after the change, and is safe.
◉ None of these programs
→ Yes. The function is safe as written, so none of these programs causes undefined behavior.
````

````quiz
Consider the original `apply_curve` method above. Of the following fixes, which best satisfies these three criteria: the fixed function passes the compiler, it
preserves the intention of the original code, and it does not introduce unnecessary inefficiencies?

○ Call `.as_ref()` on the result of `self.get_curve()`
→ Not quite. This still borrows `self` through `get_curve`, so the conflict remains.
○ Clone `self.scores` before the loop, and iterate over the clone
→ Not quite. The loop would change a copy, so the struct's scores would never be updated.
○ Iterate over `self.scores` with `iter()` instead of `iter_mut()`
→ Not quite. The loop would only read the scores, so nothing would be curved.
◉ Copy `self.curve` into `if let Some(curve) = self.curve`, and use `curve` in the loop
→ Yes. `Option<usize>` is `Copy`, so no borrow of `self` is held during the loop, and the scores are changed in place.
````

---

*For more detail, see the Ownership Inventory in [The Rust Programming Language](https://doc.rust-lang.org/book/ch10-04-inventory.html).*
