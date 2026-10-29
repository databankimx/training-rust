## Summary: Pattern Syntax

A pattern describes the shape of a value, and binds names to its parts. The same syntax works in every place a pattern is used.

* **Literals, ranges, and `|`:** each arm can test a single value, a range, or several alternatives. *(See the literals and ranges step.)*
* **`@` and `_`:** `@` binds a name while testing a range, and `_` ignores a value. *(See the literals and ranges step.)*
* **Structs:** a struct pattern names the fields it needs, and `..` skips the rest. *(See the destructuring structs step.)*
* **Guards:** an `if` on an arm adds a condition, checked after the pattern matches. *(See the destructuring structs step.)*

Patterns are checked in order, so put specific arms first and the catch-all last.

---

### Check your understanding

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Does this program compile? If it does, what does it print?

```rust
fn main() {
    let x = (0, 1);
    match x {
        (_, y) if y == 0 => println!("A"),
        (0, _) => println!("B"),
        _ => println!("C"),
    }
}
```

○ This program does not compile
○ This program compiles, and prints `A`
◉ This program compiles, and prints `B`
○ This program compiles, and prints `C`
→ Yes. The first arm's guard fails, because `y` is 1, not 0. The second arm matches, because the first element is 0, so `B` is printed.
````

Consider this program:

```rust,ignore
let a = [(0, 1)];
let ?? = a;
```

Is each pattern valid in place of `??`?

````quiz
The pattern `_`.

◉ Yes
→ Yes. `_` matches any value, so it's valid for any `a`.
○ No
→ Not quite. `_` is valid in any position where a pattern is allowed.
````

````quiz
The pattern `[(n, ..)]`.

◉ Yes
→ Yes. `a` is an array with one tuple, and `(n, ..)` matches a tuple by its first element.
○ No
→ Not quite. `a` is an array of tuples, so an array pattern containing a tuple pattern is valid.
````

````quiz
The pattern `[..]`.

◉ Yes
→ Yes. `..` matches any number of array elements, so this pattern is valid.
○ No
→ Not quite. `[..]` matches the whole array, so it's valid.
````

````quiz
The pattern `(_, n)`.

○ Yes
→ Not quite. `a` is an array of tuples, not a tuple, so a tuple pattern doesn't fit it.
◉ No
→ Yes. The pattern is a tuple, and `a` is an array, so the types don't match.
````

Does this program compile? If not, on which line does Rust report the problem?

```rust,ignore
fn main() {
    let x = Some(&[0, 1]);
    match x {
        Some(&[.., 1, ..]) => println!("A"),
        Some(&[0, 1]) | None => println!("B"),
        _ => println!("C"),
    }
}
```

````quiz
Does the program compile?

○ This program compiles, and prints `A`
○ This program compiles, and prints `B`
○ This program compiles, and prints `C`
◉ This program does not compile, on line 4
→ Yes. Using `..` on both sides of the array pattern is ambiguous, so the compiler rejects line 4.
````

Say we have a `Point` type with fields `x` and `y`, and the pattern:

```rust,ignore
Point { x: x @ 0..=5, y } if x + y != 0
```

Does this value match the pattern?

````quiz
`Point { x: 5, y: -4 }`

◉ Yes
→ Yes. `x` is 5, which is in the range, and `x + y` is 1, which isn't zero.
○ No
→ Not quite. `x` is in the range 0 to 5, and `x + y` is not zero, so the pattern matches.
````

````quiz
`Point { x: 0, y: 1 }`

◉ Yes
→ Yes. `x` is 0, which is in the range, and `x + y` is 1, which isn't zero.
○ No
→ Not quite. `x` is in the range, and `x + y` is not zero, so the pattern matches.
````

````quiz
`Point { x: -1, y: 2 }`

○ Yes
→ Not quite. `x` is -1, which is outside the range 0 to 5, so the pattern fails.
◉ No
→ Yes. `x` must be between 0 and 5, and -1 isn't.
````

````quiz
`Point { x: 3, y: -3 }`

○ Yes
→ Not quite. `x + y` is zero, so the guard fails and the pattern doesn't match.
◉ No
→ Yes. `x + y` is 0, which the guard rules out.
````

---

*The summary is written for this course, and draws on section 19.3 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html).*
