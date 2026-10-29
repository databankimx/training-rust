## Custom Types for Validation

Some functions need values that are always valid. Checking the value in every function is tedious, and easy to forget. Rust's type system can do the
checking once, if the type itself can only be built from valid values.

---

### The idea

A type can keep its field private, and provide one constructor that checks the value. Every instance of the type then comes through that check. Functions
that take the type can rely on the value without checking it again.

The sample defines a `Guess` type for a number between 1 and 100. The field `value` is private, so code outside the module can't set it directly:

```rust,ignore
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("The guess value must be between 1 and 100, got {value}.");
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

The constructor `new` panics for a value out of range. That's a fit for panicking, because a value out of range is a bug in the code that calls `new`, not
something the caller is expected to handle. The method `value` is a getter, and it's how callers read the number.

---

### The sample

The sample lives in two files. The module `guessing_game` is in `src/guessing_game.rs`, and `main.rs` declares it with `mod guessing_game;`. The file selector
above the editor lets you see both. `main` creates a valid guess and prints it:

```
Your guess is 42
```

Any function that takes a `Guess` can rely on its value being between 1 and 100, without checking.

---

### Panic or Result

Whether a constructor like this should panic or return a `Result` depends on who is at fault. A value outside the range is a bug in the calling code, so
panicking alerts the programmer. A value that comes from user input is expected to be wrong sometimes, so the input should be checked, and an error returned
for the user to act on. The rule of thumb is that expected failures return a `Result`, and contract violations panic.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Your guess is 42
```

The comments in `src/main.rs` suggest changing 42 to 200. The program then panics with the message from `new`, which names the range and the value.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
A Rust programmer is designing a library for writing command-line interfaces. As part of this library, they are implementing a function to parse
command-line flags provided by a user. Which implementation would be most appropriate for this domain?

```rust
fn parse_flag_v1(flag: &str) -> Result<String, String> {
  match flag.strip_prefix("--") {
    Some(no_dash) => Ok(no_dash.to_string()),
    None => Err(format!("Invalid flag {flag}"))
  }
}

fn parse_flag_v2(flag: &str) -> String {
  match flag.strip_prefix("--") {
    Some(no_dash) => no_dash.to_string(),
    None => panic!("Invalid flag {flag}")
  }
}
```

◉ `parse_flag_v1`
→ Yes. A user can mistype a flag, which is an expected failure. Returning an `Err` lets the library show help, rather than stopping the program with a panic.
○ `parse_flag_v2`
→ Not quite. A mistyped flag is expected from users, so it should be a recoverable error, not a panic.
````

---

*For more detail, see section 9.3 "To panic! or Not to panic!" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html).*
