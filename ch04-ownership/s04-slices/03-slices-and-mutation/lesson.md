## Slices and Mutation

Now that we have string slices, we can rewrite the function from the first step so that it returns a slice. This
turns the index problem into a compile-time check.

---

### The rewritten function

The function returns a `&str`, a reference to the first word, rather than a number:

```rust,ignore
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

When the function finds a space, it returns a slice from the start of the string up to that space. If there's no space,
the whole string is the first word, so it returns a slice of the whole string.

The result is a reference tied to the underlying data. It's a pointer and a length, so it knows exactly which bytes it
covers. You no longer have to keep a separate index in sync with the string.

The sample for this step uses the function to print the first word:

```
the first word is: hello
```

---

### The compiler now catches the bug

Here's the program from the first step, with the slice version of the function. It keeps the word, and tries to clear the
string before the word is printed:

```rust,ignore
fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    println!("the first word is: {}", word);
}
```

Calling `first_word` now borrows `s`. That removes the write permission from `s`, so `s.clear()` is rejected. The compiler
says so:

```text
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let word = first_word(&s);
  |                           -- immutable borrow occurs here
5 |
6 |     s.clear();
  |     ^^^^^^^^^ mutable borrow occurs here
7 |     println!("the first word is: {}", word);
  |                                      ---- immutable borrow later used here
```

The line numbers may differ. The message is the important part: the slice in `word` is an immutable borrow of `s`, and it's
still in use when `clear` needs a mutable one.

This is the benefit the book describes. The bug from the first step, where the index went out of sync, can't happen
any more. The compiler finds it at the point where it would have happened, and tells you why.

---

### Mutation inside a loop

Slices borrow in the same way when you iterate over them. This program tries to change a string while iterating over its
bytes:

```rust,ignore
fn main() {
    let mut s = String::from("hello");
    for &item in s.as_bytes().iter() {
        if item == b'l' {
            s.push_str(" world");
        }
    }
    println!("{s}");
}
```

The call `s.as_bytes()` creates an immutable borrow of `s`, and the loop keeps it alive. So `push_str` inside the loop would
change data that is still borrowed, and the compiler rejects it.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
the first word is: hello
```

The comments in `src/main.rs` suggest moving `s.clear()` to before the `println!`. Rust rejects that change, and the error
is the one shown above.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Does this program compile?

```rust
fn main() {
  let mut s = String::from("hello");
  for &item in s.as_bytes().iter() {
    if item == b'l' {
      s.push_str(" world");
    }
  }
  println!("{s}");
}
```

○ This program does compile
→ Not quite. `s.as_bytes()` creates an immutable borrow of `s` that the loop keeps alive, so `push_str` inside the loop is not allowed.
◉ This program does not compile
→ Yes. The loop borrows `s` through `as_bytes`, so changing `s` inside the loop breaks the borrow rules.
---
On which line does Rust report the problem?

[ 5 ]
→ Yes. Line 5 is the `push_str` call, which needs a mutable borrow of `s` while the loop still holds an immutable one.
````

---

*For more detail, see section 4.4 "The Slice Type" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch04-04-slices.html).*
