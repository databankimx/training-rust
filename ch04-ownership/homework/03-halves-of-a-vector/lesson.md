## Homework: Halves of a Vector

**The problem:** Write a function that splits a vector into two halves, and returns a mutable reference to each half.

Your function should have this signature:

```rust,ignore
fn halves(v: &mut Vec<i32>) -> (&mut [i32], &mut [i32])
```

Then use it to change the first element of the left half, and the first element of the right half, and print the vector.

Try writing the function yourself before you look at the example.

---

### Hints

* The middle position is `v.len() / 2`.
* Rust won't let you take two mutable references to overlapping parts of a vector. Think about the standard library function that
  divides a slice into two parts that can be borrowed separately.
* Explain in your own words why the compiler accepts `split_at_mut`, when it rejects `&mut v[0]` and `&mut v[2]` at the same time.
  The fixing ownership errors step has the answer.
* Think about what would happen if the two halves overlapped. Why does the design of the function rule that out?

---

### Test your function

For the vector `[1, 2, 3, 4, 5, 6]`, the left half is `[1, 2, 3]` and the right half is `[4, 5, 6]`. After changing the first element
of each half, using `left[0] += 10` and `right[0] *= 10`, the vector should be:

```
[11, 2, 3, 40, 5, 6]
```

Check the result for a vector with an odd length too. Which half is larger in that case?

---

### Extensions

* Write a version that returns the halves as slices of `&[i32]`, not `&mut`. What changes in the signature, and what can the caller do?
* Write a function that splits the vector into three parts, and returns three mutable slices.
* Try writing the function without `split_at_mut`, using indexes and two `&mut` borrows. What does the compiler say?

---

*This exercise is not from the book. It practises `split_at_mut` and mutable references from Chapter 4.*
