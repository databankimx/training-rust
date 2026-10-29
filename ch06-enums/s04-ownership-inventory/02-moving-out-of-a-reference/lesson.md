## Example: A Function That Moves Out of a Reference

This is the second program of the Ownership Inventory. It's a function that returns the string inside an option, or an empty string if
there is none. It looks reasonable, and it fails because of a move that the reader might not notice.

---

### The program

Here is the function as it first appears. It checks for `None` first, then calls `unwrap` on the reference:

```rust,ignore
/// Gets the string out of an option if it exists, returning a default otherwise
fn get_or_default(arg: &Option<String>) -> String {
    if arg.is_none() {
        return String::new();
    }
    let s = arg.unwrap();
    s.clone()
}
```

The compiler rejects this with an error:

```text
error[E0507]: cannot move out of `*arg` which is behind a shared reference
```

`unwrap` takes `self` by value, so calling it moves the `Option<String>` out of `*arg`. But `arg` is a shared reference, and the caller still owns
the option. Moving it out would leave the caller's option empty, so Rust doesn't allow it.

---

### The fix

Match on the reference instead. Matching on `&Option<String>` borrows the contents, so `s` is a `&String`, and `s.clone()` copies only the string:

```rust,ignore
fn get_or_default(arg: &Option<String>) -> String {
    match arg {
        None => String::new(),
        Some(s) => s.clone(),
    }
}
```

The sample calls the function with a value and with `None`:

```
Rust
Some("Rust")
[]
```

The first line is the copied string. The second shows that `opt` is still whole, because nothing was moved out of it. The third is an empty string,
printed inside brackets.

---

### Why the match is better than is_none and unwrap

The original combined `is_none` and `unwrap` as two separate steps, and the second one moved the data. A `match` does both checks in one step, and it
pushes the reference inward automatically, so `s` is a reference to the string. The inventory calls the `is_none` and `unwrap` pair an anti-pattern for that
reason.

Changing the parameter to `Option<String>` would also remove the error, but it would make every caller give up ownership of their option. The `match` keeps the
signature that callers expect.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Rust
Some("Rust")
[]
```

The comments in `src/main.rs` suggest putting back the original `is_none` and `unwrap` version. The compiler reports the move error from above.

---

### Quiz

*These questions are adapted from the Ownership Inventory in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Consider the original `get_or_default` function above. If you tried to compile it, which of the following best describes the compiler error you would get?

○ cannot call `arg.is_none()` without dereferencing `arg`
→ Not quite. Calling `is_none` through a reference is allowed, so the error is elsewhere.
○ `arg` does not live long enough
→ Not quite. `arg` is a borrow of a value the caller owns, so it lives long enough.
○ cannot return `s.clone()` which does not live long enough
→ Not quite. `s.clone()` makes an owned `String`, which can be returned.
◉ cannot move out of `arg` in `arg.unwrap()`
→ Yes. `unwrap` takes ownership of the option, and `arg` is only a reference, so the move isn't allowed.
````

````quiz
Consider the original `get_or_default` function above. Assume that the compiler did not reject it. Which of the following programs would (1) pass the compiler,
and (2) possibly cause undefined behavior if executed?

```rust
// Program A
let opt = Some(String::from("Rust"));
get_or_default(&opt);
println!("{:?}", opt);

// Program B
let opt = Some(String::from("Rust"));
let s = get_or_default(&opt);
println!("{}", s);

// Program C
let opt = Some(String::from("Rust"));
get_or_default(&opt);
```

○ Program A only
→ Not quite. Program B and Program C also trigger the same problem.
○ Program B only
→ Not quite. Program A and Program C also trigger the same problem.
○ Program C only
→ Not quite. Program A and Program B also trigger the same problem.
◉ All of these programs
→ Yes. If `unwrap` were allowed, the string would have two owners, `s` and `opt`, and it would be freed twice.
````

````quiz
Which of the following is the best fix for the original `get_or_default` function? The best fix passes the compiler, keeps the intention of the original code,
and avoids unnecessary work.

○ Change the parameter to `arg: &mut Option<String>`, and call `arg.as_mut().unwrap()`
→ Not quite. This changes what callers must provide, and `unwrap` still moves the data out of the option.
○ Change the parameter to `arg: Option<String>`, and keep the body as it is
→ Not quite. This compiles, but it forces every caller to give up ownership of their option, which is a more restrictive API.
○ Change the parameter to `arg: &Option<&str>`, and return `s.to_string()`
→ Not quite. This changes the type of the option the caller must pass, so it's less flexible than the original.
◉ Replace the `is_none` and `unwrap` checks with a `match` on `arg`
→ Yes. A `match` on the reference borrows the contents, keeps the caller's signature, and does both checks in one step.
````

---

*For more detail, see the Ownership Inventory in [The Rust Programming Language](https://doc.rust-lang.org/book/ch06-04-inventory.html).*
