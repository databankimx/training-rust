## Matching Option

`Option<T>` is an enum, so `match` works on it exactly as it did on `Coin`. The `Some` variant holds a value, and the `None` variant doesn't.
Matching lets you handle each case, and use the value only in the case where it exists.

---

### The plus_one function

The sample takes an `Option<i32>`. If there's a value, it adds one. If there isn't, it returns `None`, without doing any arithmetic:

```rust,ignore
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```

The `Some(i)` arm binds `i` to the number inside. The `None` arm has no data, so it returns `None` directly. The sample calls the function twice:

```rust,ignore
let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

Its output is:

```
Some(6) None
```

When `x` is `Some(5)`, the first arm doesn't match, so the second arm runs, and `i` is `5`. When `x` is `None`, the first arm matches, so the value of the
match is `None`, and no other arm is checked.

---

### Why this is safe

Because `Option<i32>` and `i32` are different types, the compiler makes you handle both cases before you use the number. You can't forget the `None`
arm, and that's what prevents the kind of bug that `null` causes in other languages. The next step covers this in detail.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Some(6) None
```

The comments in `src/main.rs` suggest swapping the two arms. The program still gives the same output, because each arm has its own pattern. The
order matters only when patterns overlap, as they did in the coin-matching quiz in the first step.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Consider this method implemented for the `Option` type:

```
impl<T> Option<T> {
  fn unwrap_or(self, other: T) -> T {
    match self {
      Some(t) => t,
      None => other
    }
  }
}
```

Which sentence best describes the behavior of this function?

○ Returns a new option containing the object inside `self` if it exists, and `other` otherwise
→ Not quite. The function returns the value itself, not a new `Option`.
○ Inserts `other` into `self` if `self` does not already contain a value
→ Not quite. The function doesn't change `self`. It returns a value.
○ Returns a reference to the object inside `self` if it exists, and `other` otherwise
→ Not quite. The function takes `self` by value, so it returns the value itself, not a reference to it.
◉ Returns the object inside `self` if it exists, and `other` otherwise
→ Yes. It consumes `self`, returns the inner value if there is one, and returns `other` otherwise. The standard library has a function like this.
````

````quiz
Consider these two implementations of a function to decrement an unsigned number twice.

```
fn decr_twice_v1(n: u32) -> Option<u32> {
  match n {
    0 => None,
    1 => None,
    n2 => Some(n2 - 2)
  }
}

fn decr_twice_v2(n: u32) -> Option<u32> {
  if n == 0 {
    None
  } else if n == 1 {
    None
  } else {
    Some(n - 2)
  }
}
```

The functions have the same behavior for:

○ No inputs
→ Not quite. For each input, the two functions return the same result.
○ Some, but not all inputs
→ Not quite. A `match` on a number and an `if` chain that checks equality do the same thing here.
◉ All inputs
→ Yes. A `match` on a value is a specialized `if` that checks for equality, so both versions behave identically for every input.
````

---

*For more detail, see section 6.2 "The match Control Flow Construct" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch06-02-match.html).*
