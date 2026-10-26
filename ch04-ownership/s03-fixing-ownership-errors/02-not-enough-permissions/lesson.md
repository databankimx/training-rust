## Not Enough Permissions

Sometimes the borrow checker rejects a function because it asks for more permission than the function needs.
A common case is trying to change data that was only lent as read-only.

Suppose you want a function that builds a person's full name from a vector of name parts, with a title added at the
end. The function is supposed to take the parts, and produce the full name, such as `"Ferris Jr. Esq."`.

---

### The error

Here's a first attempt, which pushes the title onto the vector:

```rust,ignore
fn stringify_name_with_title(name: &Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}
```

The borrow checker rejects it, because `name` is an immutable reference, but `push` needs the write permission.
Write permission would mean changing a vector that the caller lent out as read-only.

---

### Is the program unsafe?

Yes, and the reason is important. The caller might hold other references into `name`. Here's how that could go wrong:

```rust,ignore
fn main() {
    let name = vec![String::from("Ferris")];
    let first = &name[0];
    stringify_name_with_title(&name);
    println!("{}", first);
}
```

`first` refers to the first element of `name`. If `push` reallocates the vector to make room, `first` is left pointing at
freed memory, and the `println!` reads it. So the function is unsafe, and the borrow checker is right to stop it.

---

### Two fixes that aren't good

The book shows two ways to make the error go away. Both silence the compiler, but both change the function in ways
that cause problems for its callers.

**Change the input to `&mut Vec<String>`.** This compiles, but a caller who calls `stringify_name_with_title` would
not expect its vector to change. **Functions should not mutate their inputs if the caller would not expect it.**
Another function, such as `add_title_to_name`, might be expected to change its input. This one isn't.

**Take ownership, with `Vec<String>`.** This also compiles, but it makes the caller's vector unusable after the call. It's
very rare for a Rust function to take ownership of a heap-owning collection like `Vec` or `String` when it only needs to read it.

---

### The fix that works

The best choice is the one we already have: `&Vec<String>`. The function only needs to read its input, so the read
permission is enough. The problem is inside the body, which tries to change the input. So we change the body.

One possibility is to clone the input, and mutate the copy:

```rust,ignore
fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    full
}
```

That works, but the clone copies every string. A better version adds the title after the join, which already builds a new
string:

```rust,ignore
fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}
```

This version is the sample for this step. `join` copies the parts into the new string `full`, so only that new string is
changed. The caller's vector is never touched, and only the read permission is needed.

In general, writing Rust functions is a balance: ask for the *right* level of permission. For this example, the
most idiomatic choice is to expect only read permission on `name`.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Ferris Jr. Esq.
```

The comments in `src/main.rs` suggest changing the function to take `&mut Vec<String>`. Try it, and check whether `main`
still compiles without any change. Then ask whether a caller should expect its vector to be changed.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Let's say a programmer tried writing the following function:

```
/// Returns a person's name with "Ph.D." added as a title
fn award_phd(name: &String) -> String {
    let mut name = *name;
    name.push_str(", Ph.D.");
    name
}
```

The Rust compiler rejects their code with the following error:

```text
error[E0507]: cannot move out of `*name` which is behind a shared reference
```

Given the stated purpose of the function, which of the following would be the most idiomatic fix to the program?

○ `let mut name = &*name;`
→ Not quite. This still leaves `name` as a reference, so `push_str` would still need a mutable borrow of data that is only lent as read-only.
◉ `let mut name = name.clone();`
→ Yes. The function must not change the caller's name, so cloning the data and mutating the local copy is the idiomatic fix.
○ Change the parameter to `mut name: String`
→ Not quite. This would take ownership of the caller's string, which makes it unusable after the call.
○ Change the parameter to `name: &mut String` and return nothing
→ Not quite. This changes the caller's original name, which the function is not meant to do.
````

````quiz
Let's say a programmer tried writing the following function:

```
/// Rounds all the floats in a vector to the nearest integer, in-place
fn round_in_place(v: &Vec<f32>) {
    for n in v {
        *n = n.round();
    }
}
```

The Rust compiler rejects their code with the following error:

```text
error[E0594]: cannot assign to `*n`, which is behind a `&` reference
```

Given the stated purpose of the function, which of the following would be the most idiomatic fix to the program?

○ Return a new `Vec<f32>` that contains the rounded values
→ Not quite. The function is supposed to round the vector in place, so returning a copy is not the intended behaviour.
○ Change the parameter to `mut v: Vec<f32>`
→ Not quite. This takes ownership of the vector, which the caller would then lose.
◉ Change the parameter to `v: &mut Vec<f32>`
→ Yes. The function must change the caller's vector, so it needs a mutable reference to it.
○ Clone the vector inside the loop, and round the clone
→ Not quite. The clone would be changed, and the caller's vector would stay the same.
````

---

*For more detail, see section 4.3 "Fixing Ownership Errors" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch04-03-fixing-ownership-errors.html).*
