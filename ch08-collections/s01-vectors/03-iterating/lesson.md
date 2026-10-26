## Iterating Over Vectors

To look at every element of a vector, you iterate over it, rather than index it one position at a time. The `for` loop does this, and its
item can be a shared reference or a mutable reference, depending on what you loop over.

---

### Reading each element

Looping over `&v` gives an immutable reference to each element. The sample prints them:

```rust,ignore
for i in &v {
    println!("{i}");
}
```

The `i` here is a `&i32`, a reference to a value in the vector. Printing it works because formatting follows the reference.

---

### Changing each element

Looping over `&mut v` gives a mutable reference to each element. To change the value a reference points to, you use `*`, the dereference operator:

```rust,ignore
for i in &mut v {
    *i += 50;
}
```

Without the `*`, you'd be trying to change the reference, not the number it points to. The output of the whole sample is:

```
100
32
57
150
82
107
```

---

### Why you can't change the vector while looping

A loop over the vector holds a pointer into it, and the pointer moves along the elements. If the loop adds a value with `push`, the vector may need
to move to a new place in memory, and the loop's pointer would point at freed memory. Rust rejects this.

The book's example is `dup_in_place`, which tries to append each element of a vector to the vector itself:

```rust,ignore
fn dup_in_place(v: &mut Vec<i32>) {
    for n_ref in v.iter() {
        v.push(*n_ref);
    }
}
```

`v.iter()` borrows `*v` immutably for the whole loop, so `v.push` needs a mutable borrow that the loop still holds. The compiler reports:

```text
error[E0502]: cannot borrow `*v` as mutable because it is also borrowed as immutable
```

The line numbers depend on your version. The message is the important part: the loop is still borrowing the vector, and `push` needs the borrow to be over.

If you want to duplicate a vector, collect the new values first, and then add them after the loop finishes.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
100
32
57
150
82
107
```

The comments in `src/main.rs` suggest calling `v.push(1)` inside the second loop. Rust rejects that, because `v` is borrowed by the loop.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Does this program compile?

```rust
fn main() {
  let mut v = vec![1, 2, 3];
  for i in &mut v {
    v.push(*i);
  }
  println!("{} {} {}", v[3], v[4], v[5]);
}
```

○ This program does compile
→ Not quite. The loop holds a mutable borrow of `v`, so `v.push` inside the loop is not allowed.
◉ This program does not compile
→ Yes. Even though `i` is a mutable reference into `v`, the loop keeps `v` borrowed, so pushing to it is an ownership error.
---
On which line does Rust report the problem?

[ 4 ]
→ Yes. Line 4 is the `v.push` call inside the loop.
````

````quiz
Does this program compile? If it does, what does it print?

```rust
fn main() {
  let mut v: Vec<i32> = vec![1, 2, 3];
  let mut v2: Vec<&mut i32> = Vec::new();
  for i in &mut v {
    v2.push(i);
  }
  *v2[0] = 5;

  let a = *v2[0];
  let b = v[0];
  println!("{a} {b}");
}
```

○ This program does not compile
→ Not quite. Each `i` is a mutable reference into `v`, so storing them in `v2` is allowed, and the loop's borrow ends before `v` is read.
◉ This program does compile
→ Yes. `v2` holds references into `v`, so changing `*v2[0]` changes `v[0]`.
---
If it compiles, what does it print?

[ 5 5 ]
→ Yes. Setting `*v2[0]` to 5 changes the first element of `v`, so both `a` and `b` are 5.
````

---

*For more detail, see section 8.1 "Storing Lists of Values with Vectors" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch08-01-vectors.html).*
