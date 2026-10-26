## Match Expressions

`match` is a control flow construct. It compares a value against a series of patterns, and runs the code for the first pattern
that fits. It's like a coin-sorting machine: each coin falls through the first hole it fits, and the rest are never tried.

`match` is an expression, so it produces a value, and the value of the matching arm is the value of the whole `match`. Patterns can
be literal values, variable names, wildcards, and many other forms. Chapter 19 covers all of them.

---

### The coin example

The sample is the book's coin function, which returns the value of each coin in cents:

```rust,ignore
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

Each arm has a pattern, then `=>`, then the code to run. The code is an expression, so each arm here just returns a number. Arms are
separated by commas. The `match` has four arms, one for each variant of `Coin`.

Unlike `if`, the value being matched can be any type. Here it's a `Coin`, not a `bool`.

The sample calls the function for each coin:

```
Penny is worth 1 cents
Nickel is worth 5 cents
Dime is worth 10 cents
Quarter is worth 25 cents
```

---

### Arms are tried in order

When the `match` runs, it compares the value with each arm's pattern, from the top. The first pattern that matches wins, and the later
arms are never checked. This matters when patterns overlap, as the next quiz shows.

If an arm needs several lines of code, wrap it in curly brackets. The comma after the arm is then optional.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Penny is worth 1 cents
Nickel is worth 5 cents
Dime is worth 10 cents
Quarter is worth 25 cents
```

The comments in `src/main.rs` suggest deleting the `Coin::Dime` arm. Rust rejects the `match`, because it no longer covers every variant. The
next steps explain why the compiler insists on this.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Does this program compile? If it does, what does it print?

```rust
enum Location {
  Point(i32),
  Range(i32, i32)
}

fn main() {
  let l: Location = Location::Range(0, 5);
  let n = match l {
    Location::Point(_) => -1,
    Location::Range(_, 5) => 5,
    Location::Range(0, _) => 0,
    _ => -2
  };
  println!("{n}");
}
```

○ This program does not compile
→ Not quite. The program is valid, and every pattern is allowed.
◉ This program does compile
→ Yes. The `match` has a catch-all `_` arm at the end, so every value is covered.
---
If it compiles, what does it print?

[ 5 ]
→ Yes. Both `Location::Range(_, 5)` and `Location::Range(0, _)` match this value, but the first one that matches is used, so the result is `5`.
````

---

*For more detail, see section 6.2 "The match Control Flow Construct" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch06-02-match.html).*
