## Summary: Unsafe Rust

Unsafe Rust is the same language, with five extra operations. It's used when you know code is correct, and the compiler can't prove it.

* **Superpowers:** dereferencing raw pointers, calling unsafe functions, using mutable statics, implementing unsafe traits, and reading union fields. *(See the raw pointers and split step.)*
* **Safe abstractions:** wrap unsafe code in a function that is safe to call, and make the checks that keep it correct. *(See the raw pointers and split step.)*
* **Mutable statics and FFI:** both need `unsafe`, and `SAFETY` comments record the requirements. *(See the mutable static and calling C steps.)*
* **Checking:** Miri runs a program and reports undefined behavior it can detect. *(See the text of section 20.1.)*

Keep unsafe blocks small, and document why each one is safe. A bug in unsafe code can only come from one of these blocks, which makes it easier to find.

---

### Check your understanding

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Which of the following is one of the five "superpowers" that Rust enables inside an `unsafe` block?

◉ Dereferencing a raw pointer
○ Disabling the borrow checker
○ Converting a reference to a raw pointer
○ Using a mutable reference twice
→ Yes. Dereferencing a raw pointer is one of the five superpowers. The borrow checker stays on in unsafe blocks, and converting a reference to a raw pointer is already allowed in safe code.
````

Does this program compile? If so, what does it print?

```rust,ignore
fn main() {
    let mut v = Vec::with_capacity(4);
    for i in 0..3 {
        v.push(i);
    }
    let n = &raw const v[0];
    v.push(4);
    println!("{}", unsafe { *n });
}
```

````quiz
Program result

○ This program does not compile
○ This program compiles, and prints `3`
◉ This program compiles, and prints `0`
○ This program compiles, and prints `4`
→ Yes. The vector has room for four elements, so pushing the fourth doesn't move the data, and the pointer still points at the first element, which is 0. This is only safe because of the capacity. With a capacity of 3, the push would reallocate, and `n` would dangle.
````

Which of these situations is an idiomatic reason to use `unsafe` code, or a safe wrapper around it, to work around the borrow checker? Answer each one.

````quiz
Getting two mutable references to disjoint indices in an array

◉ Yes
→ Yes. The borrow checker can't see that the indices don't overlap, so `split_at_mut` uses unsafe code.
○ No
→ Not quite. This is a standard case: the borrow checker is conservative, and the two references don't overlap.
````

````quiz
Allowing a value to be uninitialized when it isn't being read

◉ Yes
→ Yes. Working with uninitialized values, through `MaybeUninit`, is a recognized case where the borrow checker is too strict.
○ No
→ Not quite. `MaybeUninit` exists for exactly this case.
````

````quiz
Having a reference to one field of a struct sit in another field of the same struct

◉ Yes
→ Yes. The borrow checker can't reason about self-references, so unsafe code, usually behind `Pin`, is used for them.
○ No
→ Not quite. The borrow checker can't track self-references, so this is a valid reason.
````

````quiz
Returning a pointer to a stack-allocated variable out of a function

○ Yes
→ Not quite. A pointer to a stack variable is never valid after the function returns, so unsafe code can't make this correct.
◉ No
→ Yes. The variable is freed when the function returns, so the pointer would dangle. The fix is to return owned data, or an `Rc<T>`.
````

---

*The summary is written for this course, and draws on section 20.1 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html).*
