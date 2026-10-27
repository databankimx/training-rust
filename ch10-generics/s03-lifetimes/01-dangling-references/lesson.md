## Dangling References and Lifetimes

Every reference in Rust has a *lifetime*: the part of the program during which the reference is valid. Most of the time the compiler works out lifetimes on its
own, just as it works out types. Lifetimes need annotations only when the relationship between the references is unclear.

The main purpose of lifetimes is to prevent dangling references. A dangling reference points at data that has already been freed, so a program using it reads
or writes memory it doesn't own.

---

### The sample

The sample declares `x`, then borrows it in `r`, and prints `r`. `x` lives until the end of `main`, so `r` is valid when it's used:

```rust,ignore
let x = 5;
let r = &x;
println!("r: {r}");
```

The output is:

```
r: 5
```

Both variables live until the end of `main`, so the reference never outlives its data.

---

### The dangerous version

The book's example moves `x` into an inner block, and keeps `r` in the outer one. When the inner block ends, `x` is freed, and `r` would then point at freed
memory. Rust rejects the program with an error that says `x` does not live long enough. The borrow checker compares the scopes, and sees that `r` lasts longer
than `x`.

The fix is always to make the data live at least as long as the reference that points at it.

---

### Lifetime annotations

Lifetime names start with an apostrophe, and are usually short, like `'a`. An annotation describes the relationship between references. It doesn't change how long
any of them lives. The next step uses an annotation on a function.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
r: 5
```

The comments in `src/main.rs` suggest the inner-block version. Rust reports that `x` does not live long enough, and points at the place where `r` is used after `x`
has gone out of scope.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Which kind of programming error is a lifetime supposed to prevent?

○ Using the result of a fallible computation before checking for the possibility of error
→ Not quite. That's about handling `Result` and `Option` values, which Chapter 9 covers.
○ Indexing past the bounds of an array (buffer overflow)
→ Not quite. Bounds are checked at runtime, and lifetimes don't cover them.
○ Not allocating enough memory for an object
→ Not quite. Allocation is handled by the program and its allocator, not by lifetimes.
◉ Using a reference to an object after its memory has been freed
→ Yes. Lifetimes make sure every reference is used only while the data it points to is still valid.
````

````quiz
Does this program compile? If not, on which line does Rust report the problem?

```rust
fn shortest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
  if x.len() < y.len() {
    x
  } else {
    y
  }
}

fn main() {
  println!("{}", shortest("hello", "rust"));
}
```

○ This program compiles
→ Not quite. The function signature says it returns a reference with lifetime `'a`, but one branch returns `y`, which has lifetime `'b`.
◉ This program does not compile, on line 5
→ Yes. Line 5 returns `y`, whose lifetime `'b` isn't the `'a` promised in the return type.
○ This program does not compile, on line 9
→ Not quite. Line 9 is the start of `main`, which is fine. The problem is the return of `y` on line 5.
````

---

*For more detail, see section 10.3 "Validating References with Lifetimes" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html).*
