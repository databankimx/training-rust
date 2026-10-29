## Example: Removing Zeros From a Vector

This is the first program of the Ownership Inventory #2. The function removes every zero from a vector, in place. It looks like an ordinary loop,
and it runs into the borrow checker in an interesting way.

---

### The program

Here is the function as it first appears. It loops over the vector with `iter`, and removes elements as it goes:

```rust,ignore
/// Removes all the zeros in-place from a vector of integers.
fn remove_zeros(v: &mut Vec<i32>) {
    for (i, t) in v.iter().enumerate().rev() {
        if *t == 0 {
            v.remove(i);
            v.shrink_to_fit();
        }
    }
}
```

The compiler rejects it:

```text
error[E0502]: cannot borrow `*v` as mutable because it is also borrowed as immutable
```

`v.iter()` borrows `v` for the whole loop, and `v.remove(i)` needs to change the vector while that borrow is still used. Rust won't allow both.

---

### Why the loop would be unsafe

Assume the compiler accepted this. Then a program that calls `remove_zeros` on a vector with a zero after the first element could cause undefined
behavior. The call to `v.shrink_to_fit()` can deallocate the vector's memory, and the iterator still points at the old data. Reading through that
pointer afterwards would read freed memory.

The safety problem is inside the function itself. Reading `v` after the call isn't needed for it to happen.

---

### The fix

Loop over the indexes instead, from the end, and don't keep an iterator alive:

```rust,ignore
fn remove_zeros(v: &mut Vec<i32>) {
    for i in (0..v.len()).rev() {
        if v[i] == 0 {
            v.remove(i);
        }
    }
}
```

The range `0..v.len()` is a plain range of numbers. It doesn't borrow `v`, so `v.remove(i)` is allowed inside the loop. Walking backwards means that
removing an element at `i` never moves a value that the loop hasn't checked yet. The sample's output is:

```
[1, 2, 3]
```

The original also called `shrink_to_fit`, which isn't needed. It gives memory back, but it's a cost paid on every call, and the program is correct
without it.

The most idiomatic option is `Vec::retain`, a standard-library method that keeps only the elements that pass a test. It doesn't need an index at all.
The book's answer asks for the index-based fix, so the sample uses that.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
[1, 2, 3]
```

The comments in `src/main.rs` suggest walking forwards. The loop then reaches an index that no longer exists, because the vector got shorter, and the
program panics.

---

### Quiz

*These questions are adapted from the Ownership Inventory in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Consider the original `remove_zeros` function above. If you tried to compile it, which of the following best describes the compiler error you would get?

○ `v` does not live long enough to call `v.remove(i)`
→ Not quite. `v` is a mutable reference to a vector owned by the caller, so it lives long enough.
○ `t` cannot be dereferenced while `i` is live
→ Not quite. Dereferencing `t` is allowed, and `i` is just an index.
○ `v.iter()` cannot be called on a mutable reference
→ Not quite. `iter` can be called through a mutable reference, as long as the borrow rules are kept.
◉ `v.remove(i)` cannot borrow `v` as mutable
→ Yes. `v.iter()` borrows `v` for the whole loop, so `v.remove(i)` can't borrow it mutably inside the loop.
````

````quiz
Consider the original `remove_zeros` function above. Assume that the compiler did not reject it. Which of the following programs would (1) pass the compiler,
and (2) possibly cause undefined behavior if executed?

○ `let mut v = vec![1; 100];` followed by `remove_zeros(&mut v);`
→ Not quite. This vector has no zeros, so `remove` is never called, and nothing is freed.
○ None of these programs
→ Not quite. A vector with a zero after the first element triggers the problem.
◉ The programs that call `remove_zeros` on `vec![5, 5, 0]` and on `vec![1, 2, 0, 3]`
→ Yes. Both vectors have a zero after the first element. `shrink_to_fit` then frees memory the iterator still points at.
````

````quiz
Consider the original `remove_zeros` function above. Of the following fixes, which best satisfies these three criteria: the fixed function passes the compiler,
it preserves the intention of the original code, and it does not introduce unnecessary inefficiencies?

○ Clone the vector inside the function, and iterate over the clone
→ Not quite. This allocates a copy of the whole vector, which is an unnecessary cost.
○ Return a new vector that keeps only the non-zero values
→ Not quite. This changes the signature, so the caller no longer gets the vector changed in place.
○ Change the parameter to take the vector by value, `v: Vec<i32>`
→ Not quite. The caller would lose the vector, which changes what the function is for.
◉ Loop over the indexes `(0..v.len()).rev()`, and check `v[i]` directly
→ Yes. The range doesn't borrow `v`, so `remove` is allowed, and no extra allocation is made.
````

---

*For more detail, see the Ownership Inventory in [The Rust Programming Language](https://doc.rust-lang.org/book/ch08-04-inventory.html).*
