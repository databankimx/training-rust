## Reading Elements of Vectors

There are two ways to read a value from a vector: indexing with `[]`, and the `get` method. They behave the same for a valid index, and
differ when the index is out of range. Choosing between them is a decision about what your program should do in that case.

---

### Indexing

Indexing with `&v[2]` gives a reference to the element at that position. Vectors count from zero, so index 2 is the third element. The sample
prints it:

```rust,ignore
let third: &i32 = &v[2];
println!("The third element is {third}");
```

Indexing panics when the index is outside the vector. The program stops, and the message reports the length of the vector and the index used. For
example, indexing a five-element vector at 100 panics with:

```text
index out of bounds: the len is 5 but the index is 100
```

Use indexing when an out-of-range index means the program has a bug, and it should stop.

---

### get

The `get` method returns an `Option<&T>`. It gives `Some` with a reference when the index is valid, and `None` when it isn't:

```rust,ignore
match v.get(100) {
    Some(value) => println!("Element 100 is {value}"),
    None => println!("There is no element at index 100"),
}
```

The sample's output is:

```
The third element is 3
There is no element at index 100
```

Use `get` when an out-of-range index can happen in normal use. For example, if a user types a number, `get` lets you tell them how many items there
are, and ask again, instead of crashing.

---

### Borrowing and the vector

A reference from `&v[2]` or `v.get(2)` borrows the vector, so the borrow checker applies its normal rules. You can't hold an immutable reference
to an element and then `push` to the vector, while the reference is still used. Adding a value may need to move the whole vector to a larger place
in memory, and the reference would then point at memory that has been freed. The next step shows a related error, which comes up while iterating.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
The third element is 3
There is no element at index 100
```

The comments in `src/main.rs` suggest indexing element 100 directly. The program panics with the message shown above.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Which call to this `find_until` function will cause a runtime panic?

```rust
fn find_until(v: &Vec<i32>, n: i32, til: usize) -> Option<usize> {
  for i in 0 .. til {
    if v[i] == n {
      return Some(i);
    }
  }
  return None;
}
```

○ `find_until(&vec![1, 2, 3], 0, 0)`
→ Not quite. With `til` set to 0, the loop never runs, so nothing is indexed.
○ `find_until(&vec![1, 2, 3], 3, 3)`
→ Not quite. The loop checks indexes 0, 1, and 2, which are all valid for a vector of length 3. It finds the 3 at index 2 and returns.
○ `find_until(&vec![1, 2, 3], 1, 4)`
→ Not quite. The 1 is at index 0, so the function returns before it reaches index 3.
◉ `find_until(&vec![1, 2, 3], 4, 4)`
→ Yes. The value 4 is never found, so the loop runs through index 3, which is outside a vector of length 3, and `v[3]` panics.
````

---

*For more detail, see section 8.1 "Storing Lists of Values with Vectors" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch08-01-vectors.html).*
