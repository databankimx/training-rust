## Concatenating Strings

Two strings can be joined with `+` or with the `format!` macro. They behave differently, and the difference is about ownership.

---

### The + operator

The `+` operator joins a `String` and a string slice, and produces a new `String`:

```rust,ignore
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;
```

The `+` calls the `add` method, which takes `self` by value. So `s1` is moved into the call, and can't be used afterwards. The right-hand side must be a
string slice, so `&s2` is written with an `&`. Rust converts the `&String` to a `&str` automatically, which the book covers in Chapter 15.

`s2` is only borrowed, so it's still valid. The sample's output is:

```
Hello, world!
world!
```

`let s3 = s1 + &s2;` looks like it copies both strings, but the work is smaller. The `add` method takes `s1`, appends a copy of `s2` to it, and returns
the result. If `s1` already has room for the new text, no new allocation happens.

---

### The format! macro

If you need to join several strings, a chain of `+` becomes hard to read. The `format!` macro works like `println!`, but returns a `String`:

```rust,ignore
let s = format!("{}-{}-{}", "tic", "tac", "toe");
```

`format!` borrows all of its arguments, so none of them is moved. The sample prints `tic-tac-toe`.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Hello, world!
world!
tic-tac-toe
```

The comments in `src/main.rs` suggest using `s1` after the `+` line. Rust rejects that, because `+` moved `s1`.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
What is the difference between using `a + b` and `a.push_str(b)` to concatenate two strings?

○ `push_str` is more efficient at runtime than `+`
→ Not quite. Both append to the same buffer, and neither is more efficient in general.
○ `push_str` consumes ownership of `b`, while `+` does not
→ Not quite. `push_str` takes a slice of `b`, and doesn't move it. It's `+` that moves its left-hand side.
○ There is no difference, they are aliases for the same function
→ Not quite. `+` calls `add`, which takes `self`, and `push_str` takes `&mut self`.
◉ `+` consumes ownership of `a`, while `push_str` does not
→ Yes. `+` takes `self` by value, so `a` is moved. `push_str` takes `&mut self`, so `a` stays valid.
````

````quiz
What is the maximum number of heap allocations that could occur in this program?

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;
```

○ 6
→ Not quite. There are three calls to `String::from` and four `+` operations, which can each allocate.
◉ 7
→ Yes. Each of the three `String::from` calls can allocate, and each of the four `+` operations can too.
○ 8
→ Not quite. The count is three from `String::from` plus four from the `+` operations.
○ 9
→ Not quite. There are only three `String::from` calls and four `+` operations.
````

---

*For more detail, see section 8.2 "Storing UTF-8 Encoded Text with Strings" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch08-02-strings.html).*
