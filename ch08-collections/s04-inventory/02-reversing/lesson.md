## Example: Reversing a Vector In Place

This is the second program of the Ownership Inventory #2. It reverses a vector of strings by swapping elements from the two ends toward the middle.
The swapping is the hard part, because each swap needs two mutable borrows of the same vector.

---

### The program

Here is the function as it first appears. It swaps the elements at `i` and at `n - i - 1`:

```rust,ignore
/// Reverses the elements of a vector in-place
fn reverse(v: &mut Vec<String>) {
    let n = v.len();
    for i in 0 .. n / 2 {
        std::mem::swap(&mut v[i], &mut v[n - i - 1]);
    }
}
```

The compiler rejects it:

```text
error[E0499]: cannot borrow `*v` as mutable more than once at a time
```

Each `&mut v[...]` borrows the whole vector, because indexing borrows `v` as a whole. Two of them at once are two mutable borrows, even though the indexes
are different. The compiler doesn't reason about the values of `i` and `n - i - 1`, so it has to assume they might be the same element.

---

### Is the original safe?

Yes. Because `i` is always less than `n / 2`, and `n - i - 1` is always at least `n / 2`, the two indexes never match. So the two mutable references always
point at different elements. The program has no undefined behavior. The rejection is the compiler being cautious, not catching a real bug.

---

### The fix

The standard library has `Vec::swap`, which takes two indexes and swaps the elements at them. Inside, it does the careful work with pointers, so the
caller writes no `unsafe` code:

```rust,ignore
fn reverse(v: &mut Vec<String>) {
    let n = v.len();
    for i in 0..n / 2 {
        v.swap(i, n - i - 1);
    }
}
```

The sample's output is:

```
["c", "b", "a"]
```

The inventory's answer to the fix question is a version that uses raw pointers and `unsafe`. It passes the compiler, keeps the intent, and avoids
allocations, so it's a valid fix. But `Vec::swap` does the same job with the same efficiency, and no `unsafe` code in your program. The book describes
it as the idiomatic choice, and the rule of thumb is to use `unsafe` only when the standard library truly can't do what you need.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
["c", "b", "a"]
```

The comments in `src/main.rs` suggest replacing `v.swap` with `std::mem::swap` on two indexed references. The compiler rejects that, with the
two-mutable-borrows error from above.

---

### Quiz

*These questions are adapted from the Ownership Inventory in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Consider the original `reverse` function above. If you tried to compile it, which of the following best describes the compiler error you would get?

○ cannot borrow `v` as immutable for `v.len()` when `v` is a mutable borrow
→ Not quite. Calling `len` before the loop is allowed, because the borrow ends straight away.
○ cannot mutably borrow an element `v[i]` of a mutable vector
→ Not quite. Mutably borrowing an element of a mutable vector is allowed, as long as only one borrow is active.
◉ cannot borrow `v` as mutable twice for `v[i]` and `v[n - i - 1]`
→ Yes. Each index borrows the whole vector, and the compiler can't tell the two indexes apart.
````

````quiz
Consider the original `reverse` function above. Assume that the compiler did not reject it. Which of the following programs would (1) pass the compiler, and
(2) possibly cause undefined behavior if executed?

```rust
// Program A
let mut v = vec![String::from("a"), String::from("b")];
let x = &v[0];
reverse(&mut v);
println!("{x}");

// Program B
let mut v = vec![String::from("a"), String::from("b")];
reverse(&mut v);

// Program C
let mut v = vec![String::from("a"), String::from("b")];
reverse(&mut v);
println!("{:?}", v);
```

○ Program A
→ Not quite. Program A doesn't compile, because `reverse` needs a mutable borrow while `x` is still used.
○ Program B
→ Not quite. Program B is safe, because the two mutable references always point at different elements.
○ Program C
→ Not quite. Program C is safe, for the same reason as Program B.
◉ None of these programs
→ Yes. The function never makes two mutable references to the same element, so none of these programs causes undefined behavior.
````

````quiz
Consider the original `reverse` function above. Of the following fixes, which best satisfies these three criteria: the fixed function passes the compiler,
it preserves the intention of the original code, and it does not introduce unnecessary inefficiencies?

○ Clone the vector into `v2`, and swap elements of `v2`
→ Not quite. Cloning the whole vector is an unnecessary allocation, and the result doesn't change `v` in place.
○ Return a new vector built by popping elements from `v`
→ Not quite. This changes the signature, and the caller no longer has the vector changed in place.
○ Clone each string before swapping, and write the clones back
→ Not quite. Cloning each string allocates memory that isn't needed.
◉ Swap the two elements with raw pointers, inside an `unsafe` block
→ Yes. It passes the compiler, keeps the intent, and avoids allocations. The idiomatic choice is `Vec::swap`, which does the same job without `unsafe` in your code.
````

---

*For more detail, see the Ownership Inventory in [The Rust Programming Language](https://doc.rust-lang.org/book/ch08-04-inventory.html).*
