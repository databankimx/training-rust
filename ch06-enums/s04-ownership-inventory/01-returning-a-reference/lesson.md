## Example: A Function That Returns a Reference

This is the first program of the Ownership Inventory. It's a function that should return a separator line when the input is blank,
and the input text otherwise. The shape of the function is plausible, and the mistake in it is a common one.

---

### The program

Here is the function as it first appears. The `default` string is created inside the function, and the function returns a reference to it:

```rust,ignore
/// Makes a string to separate lines of text, returning a default if the provided string is blank
fn make_separator(user_str: &str) -> &str {
    if user_str == "" {
        let default = "=".repeat(10);
        &default
    } else {
        user_str
    }
}
```

The compiler rejects this with an error:

```text
error[E0515]: cannot return reference to local variable `default`
```

The `default` string lives on the heap, and its owner is `default`, a local variable. When the function returns, `default` is dropped, and its
heap data is freed. The returned reference would then point at freed memory. Rust won't allow it.

---

### The fix

Change the return type to `String`, and let the function return an owned value in both branches:

```rust,ignore
fn make_separator(user_str: &str) -> String {
    if user_str == "" {
        "=".repeat(10)
    } else {
        user_str.to_string()
    }
}
```

Now each branch returns a new `String`, and the caller owns it. No reference outlives its data. The sample calls the function both ways:

```
==========
Hello world!
```

The `""` input takes the first branch, and prints ten equals signs. The other input is copied, and prints unchanged.

---

### Why not return &str?

The inventory's own discussion explains the trade-off. Returning `String` means every call allocates, and callers can't pass in a string literal
and get back a borrowed view of it without copying. A version that returns either an owned string or a borrowed one is possible, using a smart pointer
called `Cow`, which the standard library provides. For now, the owned `String` is the clearest fix.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
==========
Hello world!
```

The comments in `src/main.rs` suggest restoring the original version, which returns `&default`. The compiler then reports the error about returning a
reference to a local variable, the same one shown above.

---

### Quiz

*These questions are adapted from the Ownership Inventory in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Consider the original `make_separator` function above. If you tried to compile it, which of the following best describes the compiler error you would get?

○ `user_str` does not live long enough
→ Not quite. `user_str` is a borrow of the caller's string, and it lives long enough. The problem is the local `default`.
○ function `make_separator` cannot return a reference of type `&str`
→ Not quite. Returning a `&str` is allowed in general. The problem is what the reference points to.
○ function `make_separator` cannot return two different references
→ Not quite. Both branches return a reference, which is fine. The problem is the reference in the first branch.
◉ cannot return reference to local variable `default`
→ Yes. `default` is dropped when the function returns, so the reference would point at freed memory.
````

````quiz
Consider the original `make_separator` function above. Assume that the compiler did not reject it. Which of the following programs would (1) pass the
compiler, and (2) possibly cause undefined behavior if executed?

○ `println!("{}", make_separator("Hello world!"));`
→ Not quite. This input takes the `else` branch, which returns `user_str` unchanged. That's a valid reference to the caller's string.
◉ `let s = make_separator(""); println!("{s}");`
→ Yes. The empty string takes the `if` branch, which returns a reference to `default`. That's freed when the function returns, so `s` dangles.
○ None of these programs
→ Not quite. The empty-string call reaches the dangling reference.
````

````quiz
Which of the following is the best fix for the original `make_separator` function? The best fix passes the compiler, keeps the intention of the original
code, and avoids unnecessary work.

○ Keep `&str` as the return type, and return `&user_str` in the `else` branch
→ Not quite. `user_str` is already a `&str`, so `&user_str` is a reference to a reference. The `if` branch still returns a reference to a local.
○ Take `user_str` as a `String`, and return a `String` from both branches
→ Not quite. This compiles, but it forces every caller to create a `String`, which is less flexible than the original signature.
◉ Return `String` from both branches, and use `user_str.to_string()` in the `else` branch
→ Yes. Each branch returns an owned value, so no reference outlives its data. The `else` branch copies only when needed, and the signature still accepts a `&str`.
````

---

*For more detail, see the Ownership Inventory in [The Rust Programming Language](https://doc.rust-lang.org/book/ch06-04-inventory.html).*
