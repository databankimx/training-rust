## Checking for Panics with should_panic, and Using Result in Tests

Some code should panic in certain cases. The `Guess` type from Chapter 9 is an example: creating a guess outside 1 to 100 is a bug in the caller, and must
panic. A test can check that behavior with the `should_panic` attribute. The test passes if the code panics, and fails if it doesn't.

---

### should_panic

The sample's `Guess::new` panics for values below 1 or above 100. The test checks the upper case:

```rust,ignore
#[test]
#[should_panic(expected = "less than or equal to 100")]
fn greater_than_100() {
    Guess::new(200);
}
```

The `expected` text must be a substring of the panic message. This makes the test more precise. Without it, the test would pass if the code panicked for any
reason, including a different bug. With it, the test passes only when the panic is the one it expects.

The program's output, from **Run**, is:

```
guess: 50
```

---

### Tests that return a Result

A test can also return a `Result`. If it returns `Err`, the test fails, and `?` can be used in the body:

```rust,ignore
#[test]
fn parses_a_number() -> Result<(), std::num::ParseIntError> {
    let n: i32 = "5".parse()?;
    assert_eq!(n, 5);
    Ok(())
}
```

This is convenient when a test performs several steps that might fail. Two rules apply. First, `should_panic` can't be used on a test that returns a `Result`.
Second, to check that an operation returns `Err`, don't use `?`. Instead, match on the result, or check it with `is_err()`.

The tests, from **Test**, report two passing tests:

```
running 2 tests
test tests::greater_than_100 - should panic ... ok
test tests::parses_a_number ... ok

test result: ok. 2 passed; 0 failed; 0 ignored
```

The label `should panic` shows that the first test expects a panic.

---

### A failing should_panic test

If the bug were removed, so that `Guess::new(200)` didn't panic, the test would fail with a message saying that the test didn't panic as expected. If it panicked
with a different message, the failure would show that the expected text wasn't found.

---

### Try it in the runner

Press **Test**. The output should show two passing tests.

The comments in `src/main.rs` suggest changing the expected text to `greater than 100`. The test fails, because the real message says less than or equal to 100.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Let's say you have a function with the type signature:

```rust
fn f(x: usize) -> Result<usize, String>;
```

And you want to test that `f(0)` should return `Err(_)`. Which of the following is NOT a valid way to test that?

○ `#[test] fn test() { assert!(f(0).is_err()); }`
→ This is valid. It checks the result directly with `is_err`.
○ `#[test] #[should_panic] fn test() { f(0).unwrap(); }`
→ This is valid. The `unwrap` panics on an `Err`, and `should_panic` expects that panic.
○ `#[test] fn test() { assert!(match f(0) { Ok(_) => false, Err(_) => true }); }`
→ This is valid. A `match` turns the result into a `bool` that the assertion checks.
◉ `#[test] #[should_err] fn test() -> Result<usize, String> { f(0) }`
→ Yes. `should_err` doesn't exist in Rust. A test that returns a `Result` passes only if it returns `Ok`.
````

---

*For more detail, see chapter 11 "Writing Automated Tests" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch11-01-writing-tests.html).*
