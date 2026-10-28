## All the Places Patterns Can Be Used

Patterns appear in many places in Rust, and you've probably used them without noticing. A pattern is a shape that a value is compared with. If it matches, Rust binds any names in the
pattern to the parts of the value. This step uses a pattern in each of the main places where they appear.

**`match` arms.** Each arm has a pattern, and the value is compared with the patterns in order:

```rust,ignore
match x {
    None => println!("nothing"),
    Some(i) => println!("match: {}", i + 1),
}
```

`Some(i)` matches any `Some` value, and binds the number inside to `i`.

**`let` statements.** A `let` takes a pattern too. A plain variable name is the simplest pattern, and a tuple pattern can take a value apart in one step:

```rust,ignore
let (a, b, c) = (1, 2, 3);
```

**`if let` chains.** `if let` and `else if let` can be mixed with ordinary conditions, and the conditions don't need to relate to one another:

```rust,ignore
if let Some(color) = favorite_color {
    // ...
} else if is_tuesday {
    // ...
} else if let Ok(age) = age {
    // ...
} else {
    // ...
}
```

A `match` checks every possibility, and the compiler warns if one is missing. An `if let` chain doesn't, so a missing final `else` is not reported. That's a trade-off to keep in mind.

**`while let` loops.** The loop runs for as long as the pattern matches. Here `pop` returns `Some` until the stack is empty, and then `None` ends the loop:

```rust,ignore
while let Some(top) = stack.pop() {
    popped.push(top);
}
```

**`for` loops.** The name after `for` is a pattern. `enumerate` produces `(index, value)` pairs, and the pattern takes them apart:

```rust,ignore
for (index, value) in ['a', 'b'].iter().enumerate() {
    println!("{value} is at index {index}");
}
```

**Function parameters.** A parameter is a pattern too. This one takes a reference to a tuple, and names its two parts:

```rust,ignore
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}
```

The output of the whole sample is:

```
match: 6
let: 1 2 3
if let: purple
while let: [3, 2, 1]
a is at index 0
b is at index 1
Current location: (3, 5)
```

The `if let` chain takes the purple branch, because the age `"34"` parses as a number that's greater than 30.

---

### Try it in the runner

Press **Run**. The output should match the seven lines above.

The comments in `src/main.rs` suggest changing the age string to `"twenty"`. It can't be parsed, so the `Ok` pattern doesn't match, and the final `else` runs. The output then has `blue` where `if let: purple` was.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Does this program compile? If it does, what does it print?

```rust
fn main() {
    let mut v = vec![(1, 2), (3, 4)].into_iter();
    let mut sum = 0;
    while let Some(t) = v.next() {
        let (_, n) = t;
        sum += n;
    }
    println!("{sum}");
}
```

○ This program does not compile
○ This program compiles, and prints `3`
◉ This program compiles, and prints `6`
○ This program compiles, and prints `4`
→ Yes. Each tuple is destructured with `let (_, n) = t`, which takes the second element, so the sum is 2 + 4 = 6. The `_` ignores the first element.
````

---

*For more detail, see section 19.1 "All the Places Patterns Can Be Used" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch19-01-all-the-places-for-patterns.html).*
