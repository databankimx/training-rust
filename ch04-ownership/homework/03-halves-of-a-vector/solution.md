## Solution: Halves of a Vector

This walkthrough explains the example solution in `src/main.rs`. Read it after you've tried the exercise yourself.

### Run the solution

Press **F5**. The Output pane shows:

```
[11, 2, 3, 40, 5, 6]
```

### How it works

The function computes the middle position, then splits the vector there:

```rust,ignore
fn halves(v: &mut Vec<i32>) -> (&mut [i32], &mut [i32]) {
    let mid = v.len() / 2;
    v.split_at_mut(mid)
}
```

`split_at_mut` returns two mutable slices, one for each side of `mid`. The left slice covers indexes `0..mid`, and the right
slice covers `mid..`. Because the slices never overlap, each one can be changed without affecting the other.

The borrow checker can't see that the two halves are disjoint, which is why `&mut v[0]` and `&mut v[2]` at the same time are
rejected. Indexing borrows the whole vector. `split_at_mut` gets around that limitation, and it does so with `unsafe` code in the
standard library, which uses raw pointers. The function is safe to call because its implementation guarantees the halves don't
overlap, and the signature makes that guarantee visible to callers.

Why does the signature return two slices, instead of two indexes? Because slices carry their borrow with them. The caller can't use
`v` while `left` or `right` is alive, so the rule that prevents aliasing is still enforced.

---

*This walkthrough accompanies the exercise in Chapter 4 of the course. It's not from the book.*
