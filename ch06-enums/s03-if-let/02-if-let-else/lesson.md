## if let with else

`if let` can have an `else` branch. The `else` runs when the value doesn't match the pattern. It plays the same role as the `_` arm of
the equivalent `match`.

---

### A match and an if let side by side

Suppose you want to count the coins that aren't quarters, and announce the state of each quarter as you go. With a `match`, both cases are
written out:

```rust,ignore
match coin {
    Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    _ => count += 1,
}
```

With `if let`, the same logic reads more naturally, because the pattern and its `else` sit together:

```rust,ignore
if let Coin::Quarter(state) = coin {
    println!("State quarter from {state:?}!");
} else {
    count += 1;
}
```

Both do the same thing. The `if let` version is shorter, and makes it clear that one case is special and the other is the default.

---

### The sample

The sample runs the `if let` version over five coins:

```rust,ignore
let mut count = 0;
for coin in coins {
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
}

println!("Non-quarter coins: {count}");
```

The output is:

```
State quarter from Alaska!
State quarter from Alabama!
Non-quarter coins: 3
```

Three of the five coins aren't quarters, so the count is 3. The two quarters are announced as the loop reaches them.

---

### Choosing between match and if let

This is where the two constructs differ. An `if let` with an `else` still handles only two cases in a clear way. If you have three or more cases,
each with its own behavior, a `match` is easier to read, and the compiler checks that you covered every one.

The quiz below asks which construct suits a function that produces a value for each case. That's the situation where `match` is the right choice.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
State quarter from Alaska!
State quarter from Alabama!
Non-quarter coins: 3
```

The comments in `src/main.rs` suggest changing the `else` branch to record each coin's position. That would need a counter or a list outside the loop,
which shows the kind of state that `if let` doesn't manage for you.

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

fn get_start(loc: &Location) -> i32 {
  // return the first field of Range or the only field of Point
}
```

◉ `match`
→ Yes. The function needs to return a value for each variant, so a `match` that handles both is the most appropriate.
○ `if let`
→ Not quite. An `if let` only handles one pattern, but this function needs a result for both `Point` and `Range`.
````

---

*For more detail, see section 6.3 "Concise Control Flow with if let and let...else" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch06-03-if-let.html).*
