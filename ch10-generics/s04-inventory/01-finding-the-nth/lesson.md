## Example: Finding the nth Element

This is the first program of the Ownership Inventory #3. The function returns an element at index `n` of a slice, after sorting it. The function takes a shared
reference to the slice, so the caller expects it to be left alone. It isn't, and the reason is the first thing to understand.

---

### The program

Here is the function as it first appears:

```rust,ignore
fn find_nth<T: Ord + Clone>(elems: &[T], n: usize) -> T {
    elems.sort();
    let t = &elems[n];
    return t.clone();
}
```

The compiler rejects it with this error:

```text
error[E0596]: cannot borrow `*elems` as mutable, as it is behind a `&` reference
```

`sort` needs a mutable reference to the slice, but `elems` is a shared reference. The caller lent the slice as read-only, so the function can't change it.

---

### The fix

Make a `Vec` of references to the elements, and sort that. The elements themselves are never moved or changed:

```rust,ignore
fn find_nth<T: Ord + Clone>(elems: &[T], n: usize) -> T {
    let mut elem_refs: Vec<&T> = elems.iter().collect();
    elem_refs.sort();
    let t = elem_refs[n];
    t.clone()
}
```

`elem_refs` is a new `Vec` that holds references, so sorting it reorders only the references. Since `&T` is `Ord` whenever `T` is, `sort` works on it. Then
`clone` makes the one value the caller needs. The sample's output is:

```
3
[5, 4, 3, 2, 1]
```

The vector `v` is unchanged after the call, which is the intent of a read-only function.

---

### Why the other versions are wrong

The inventory's other candidates show the trade-offs.

* Taking `&mut [T]` changes the function's contract. The caller would have to lend the slice mutably, and a read-only query shouldn't need that.
* Copying the whole slice with `to_vec` is correct, but it costs an allocation and copies every element. A `Vec` of references is cheaper, because a reference is
  just a pointer.
* Taking the vector by value consumes the caller's data, which is not what a function that reads a value should do.

---

### Undefined behavior, and why index 10 panics

The inventory asks which programs could cause undefined behavior. None of them does. Calling `find_nth(&v, 10)` panics, because `elems[10]` is out of bounds, and
Rust checks indexes at runtime. A panic is a controlled stop, not undefined behavior. `sort` only moves elements around, so a reference to `v[0]` taken before the
call still points at a valid number afterwards, even if it's a different one.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
3
[5, 4, 3, 2, 1]
```

The comments in `src/main.rs` suggest calling `find_nth(&v, 10)`. The program panics with an index-out-of-bounds message, which is the safe outcome the inventory
describes.

---

### Quiz

*These questions are adapted from the Ownership Inventory in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Consider the original `find_nth` function above. If you tried to compile it, which of the following best describes the compiler error you would get?

○ cannot move out of shared reference for `clone`
→ Not quite. `clone` takes a reference, so calling it doesn't move anything out.
○ cannot move out of shared reference in expression `&elems[n]`
→ Not quite. Taking a reference to an element is allowed. The problem is the call before it.
○ the lifetime of `T` must outlive `&[T]`
→ Not quite. Lifetimes are not what goes wrong here.
◉ cannot borrow `elems` as mutable for `sort`
→ Yes. `sort` needs a mutable slice, and `elems` is a shared reference.
````

````quiz
Consider the original `find_nth` function above. Assume that the compiler did not reject it. Which of the following programs would (1) pass the compiler, and
(2) possibly cause undefined behavior if executed?

○ `let v = vec![5, 4, 3, 2, 1]; find_nth(&v, 10);`
→ Not quite. `elems[10]` is out of bounds, so Rust panics. A panic is a controlled stop, not undefined behavior.
○ `let v = vec![5, 4, 3, 2, 1]; find_nth(&v, 0); println!("{}", v[0]);`
→ Not quite. `sort` only moves elements, so `v[0]` is still a valid number. It may not be the same number as before, but it is safe to read.
○ `let v = vec![5, 4, 3, 2, 1]; let n = &v[0]; find_nth(&v, 0); println!("{}", n);`
→ Not quite. `n` still points at an element of `v`, because `sort` only moves elements around, so this program is safe.
◉ None of these programs
→ Yes. Rust checks indexes, and `sort` never frees or moves data out of the slice, so none of these programs causes undefined behavior.
````

````quiz
Consider the original `find_nth` function above. Of the following fixes, which best satisfies these three criteria: the fixed function passes the compiler, it
preserves the intention of the original code, and it does not introduce unnecessary inefficiencies?

○ Change the parameter to `elems: &mut [T]`
→ Not quite. This changes the contract, so the caller must lend the slice mutably, which a read-only query shouldn't require.
○ Start with `let mut elems = elems.to_vec();`
→ Not quite. This compiles and keeps the intent, but it copies every element, which is more work than necessary.
○ Change the parameter to `mut elems: Vec<T>`, and use `remove`
→ Not quite. This takes ownership of the caller's data, and `remove` changes the vector, which goes against the intent of a read-only function.
◉ Build a `Vec<&T>` of references, sort that, and clone the element at `n`
→ Yes. It passes the compiler, keeps the caller's slice unchanged, and copies only the references, plus the one value the caller needs.
````

---

*For more detail, see the Ownership Inventory in [The Rust Programming Language](https://doc.rust-lang.org/book/ch10-04-inventory.html).*
