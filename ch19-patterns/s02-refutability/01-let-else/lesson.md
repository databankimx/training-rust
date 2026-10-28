## Refutability: Patterns That Might Not Match

A pattern is *irrefutable* if it matches every possible value of its type. The `x` in `let x = 5;` is irrefutable, because it matches any value. A pattern is *refutable* if some value of its
type doesn't match it. `Some(x)` is refutable, because it doesn't match `None`.

Some places in Rust accept only irrefutable patterns: function parameters, `let` statements, and the variable in a `for` loop. There's no sensible action when a value doesn't match, so the
compiler rejects a refutable pattern there. `if let`, `while let`, and `match` arms are the places where a refutable pattern is expected, because a failed match is a normal case to handle.

This step fixes a `let` that uses a refutable pattern. The sample is `let Some(x) = some_option_value`, which would not compile with a plain `let`. The `let...else` form gives an alternative for
the case where the pattern doesn't match:

```rust,ignore
let Some(x) = some_option_value else {
    println!("no value");
    return;
};
println!("x is {x}");
```

The `else` block must leave the current scope, with `return`, `break`, `continue`, or a panic. That's what guarantees that `x` is set after the `let`. The output is:

```
x is 7
```

If `some_option_value` were `None`, the `else` block would run, and print `no value`.

---

### Try it in the runner

Press **Run**. The output should be `x is 7`.

The comments in `src/main.rs` suggest changing the value to `None`. The output is then `no value`, and the program returns before it reaches the last line.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Consider pattern-matching on an expression of some type `T`. Which of these statements best describes the difference between a refutable and an irrefutable pattern?

◉ Refutable patterns do not match some value of type `T`, while irrefutable patterns match all values of type `T`
○ Refutable patterns match some values of a different type `S`, while irrefutable patterns only match values of type `T`
○ Refutable patterns do not match any values of type `T`, while irrefutable patterns match some values of type `T`
○ Refutable patterns are patterns where `T` is an enum, while irrefutable patterns are patterns where `T` is not an enum
→ Yes. A pattern is refutable if some value of the expected type isn't matched by it. A pattern that matches every value is irrefutable.
````

The next questions use this slice:

```rust,ignore
let x: &[(i32, i32)] = &[(0, 1)];
```

Is each pattern refutable for `x`?

````quiz
The pattern `&[(x, y)]`.

◉ Yes
→ Yes. The slice might have a different number of elements, so this pattern doesn't match every value.
○ No
→ Not quite. The slice's length isn't fixed, so a pattern for exactly one element doesn't match every slice.
````

````quiz
The pattern `&[(x, y), ..]`.

◉ Yes
→ Yes. This pattern needs at least one element, so an empty slice doesn't match it.
○ No
→ Not quite. An empty slice doesn't match a pattern that asserts at least one element, so the pattern is refutable.
````

````quiz
The pattern `&[..]`.

○ Yes
→ Not quite. `..` matches any number of elements, including none, so every slice matches.
◉ No
→ Yes. `&[..]` matches every slice, so it's irrefutable.
````

````quiz
The pattern `_`.

○ Yes
→ Not quite. `_` matches any value, so it can't fail.
◉ No
→ Yes. `_` matches everything, so it's irrefutable.
````

---

*For more detail, see section 19.2 "Refutability: Whether a Pattern Might Fail to Match" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch19-02-refutability.html).*
