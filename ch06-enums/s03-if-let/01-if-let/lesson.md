## if let

`match` is the right tool when you need to handle every case. Often you only care about one pattern, and want to ignore the rest. Writing
a full `match` for that is boilerplate: you have to add an arm that does nothing, just to satisfy the compiler. `if let` is shorter for
that situation.

---

### The verbose version

Suppose you only want to print a configured maximum when there is one. With a `match`, the `None` arm exists only to do nothing:

```rust,ignore
match config_max {
    Some(max) => println!("The maximum is configured to be {max}"),
    _ => (),
}
```

The `_ => ()` arm is the boilerplate. `()` is the unit value, which means nothing, so the arm does nothing.

---

### The if let version

`if let` takes a pattern and an expression, separated by `=`. It works like a `match` with one arm:

```rust,ignore
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}
```

The pattern `Some(max)` binds `max` to the value inside. The block runs only when the value matches the pattern. The sample also tries a `None`
value, and the block is skipped:

```rust,ignore
let config_max: Option<u8> = Some(3);
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}

let no_max: Option<u8> = None;
if let Some(max) = no_max {
    println!("The maximum is configured to be {max}");
}

println!("Done");
```

Its output is:

```
The maximum is configured to be 3
Done
```

---

### What you give up

`if let` doesn't check exhaustively. A `match` fails to compile if you forget a case, but `if let` quietly ignores everything that doesn't match. That's
the trade-off: less typing, in exchange for the compiler's help. Use `if let` when ignoring the other cases is really what you mean, and a `match` when
every case needs thought.

You can think of `if let` as shorthand for a `match` that runs code for one pattern, and ignores all others.

---

### Choosing between match and if let

The quiz below asks which construct is most idiomatic in two functions. The rule is simple: if the function does something in only one case, use
`if let`. If it has to produce a result for every case, use `match`.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
The maximum is configured to be 3
Done
```

The comments in `src/main.rs` suggest adding an `else` branch to the second `if let`. The `else` runs when the value is `None`, which is the case
the `if let` alone would ignore.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Which control flow construct would be most idiomatic to use in the following function?

```rust
enum Location {
  Point(i32),
  Range(i32, i32)
}

fn print_range_max(loc: &Location) {
  // print the second field of Range, if loc is a Range
}
```

○ `match`
→ Not quite. A `match` would need an arm that does nothing for `Point`, which is boilerplate here.
◉ `if let`
→ Yes. The function only does something in one case, so `if let` is the most idiomatic choice.
````

---

*For more detail, see section 6.3 "Concise Control Flow with if let and let...else" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch06-03-if-let.html).*
