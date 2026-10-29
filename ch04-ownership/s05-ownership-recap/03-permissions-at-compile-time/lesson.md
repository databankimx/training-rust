## Permissions at Compile Time

Rust checks each operation against the permissions a variable holds at that point in the program. This step reviews the
three permissions, and the ways they change when a variable is moved or borrowed.

---

### The three permissions

* **Read** (`R`): the data can be copied.
* **Write** (`W`): the data can be mutated. A variable needs `mut` to have this.
* **Own** (`O`): the data can be moved or dropped.

If a variable doesn't have the permission an operation needs, Rust rejects the program. Here's one example, from the
earlier steps:

```rust,ignore
let n = 0;
n += 1;
```

The variable `n` isn't declared `mut`, so it has no write permission, and `n += 1` is rejected.

---

### Moves remove everything

A move of a non-copyable value, such as a `String` or a `Box`, needs the read and own permissions. After the move, the
variable has no permissions at all, which is what prevents you from using it. The moves step in section 4.1 shows this with a string passed into a function.

---

### Borrows remove some permissions

Borrowing a variable temporarily removes some of its permissions, and the borrow's own permissions depend on its kind:

* An **immutable borrow** removes write and own from the borrowed data. The reference can read the data.
* A **mutable borrow** removes read, write, and own from the borrowed data. The reference can read and write it.

The sample uses an immutable borrow, and it's accepted, because the borrow is not used after the string is changed.

The book shows several examples that the borrow checker rejects, and each is worth recognizing:

* Mutating through an immutable reference is rejected. The compiler reports that you can't borrow `*s_ref` as mutable,
  because it's behind a `&` reference.
* Mutating the borrowed data while the reference is still used is rejected. The mutation needs the write permission
  that the borrow has taken away.
* Moving data out of a reference is rejected, because a reference doesn't own what it points to.
* Accessing mutably borrowed data while the mutable reference is still in use is rejected.

The first of these produces this error:

```text
error[E0596]: cannot borrow `*s_ref` as mutable, as it is behind a `&` reference
```

The line numbers and the rest of the message depend on your version. The important part is the phrase "behind a `&`
reference", which tells you the data is only lent as read-only.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Hello
Hello
Hello world
```

The comments in `src/main.rs` suggest printing `s_ref` again after the push. Rust rejects that, because `s_ref` is still in use
when `s` is changed.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Consider the permissions in the following program:

```rust
fn main() {
    let s = String::new();
    let s_ref = &s;
}
```

Which of the following best explains why `*s_ref` does not have the own permission?

○ Ownership permits borrowing, and reborrowing `*s_ref` can cause a double-free
→ Not quite. Borrowing isn't what decides ownership here. The problem is about moving the data out.
○ Ownership permits mutation, and mutating `*s_ref` can cause a use-after-free
→ Not quite. Mutation is controlled by the write permission, not the own permission.
○ Ownership permits reading, and reading `*s_ref` can cause a use-after-free
→ Not quite. Reading is controlled by the read permission, and it doesn't free anything.
◉ Ownership permits moving, and moving out of a reference can cause a double-free
→ Yes. Only one owner of a `String` may exist. If a reference could take ownership, the string would be freed twice.
````

````quiz
The standard library has a function `slice::split_at_mut`, which lets you borrow two halves of an array mutably, even though
Rust normally disallows two mutable borrows of the same array:

```rust
fn main() {
    let mut v = vec![0, 1, 2, 3];
    let (r0, r1) = v.split_at_mut(2);
    r0[0] += 1;
    r1[0] += 1;
}
```

Which of the following best describes how it's possible to implement `split_at_mut`?

○ `split_at_mut` is a special compiler primitive that cannot be implemented within the language
→ Not quite. It's implemented in ordinary Rust, using a feature that is available to the standard library.
○ `split_at_mut` uses unsafe code to disable the borrow checker from checking the safety of mutable references
→ Not quite. The borrow checker still runs. The unsafe code is used for a narrower purpose: raw pointers.
○ `split_at_mut` calls into a C library that can't be analyzed by Rust
→ Not quite. Nothing here depends on a C library.
◉ `split_at_mut` uses unsafe code to circumvent the borrow checker with raw pointers
→ Yes. The unsafe code uses raw pointers, which the borrow checker doesn't check. The function is safe to use because its implementation guarantees the two halves don't overlap.
````

---

*For more detail, see section 4.5 "Ownership Recap" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch04-05-ownership-recap.html).*
