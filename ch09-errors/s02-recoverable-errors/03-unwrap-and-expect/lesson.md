## unwrap and expect

Matching on every `Result` is precise, but it's verbose when you're sure a value is there, or when a failure really should stop the program. Two
shortcuts exist for that case.

`unwrap` returns the value inside `Ok`. If the result is `Err`, it panics with a generic message that includes the error.

`expect` does the same, but you give the panic message yourself. It's the better choice in real code, because the message explains why the operation
was expected to succeed. If that assumption turns out to be wrong, the message helps you find the problem.

---

### The sample

The sample parses a string into an `i32`. `parse` returns a `Result`, so `expect` gives the number or panics:

```rust,ignore
let number: i32 = "42".parse().expect("the text should be a number");
println!("The number is {number}");
```

The output is:

```
The number is 42
```

The `let number: i32` annotation tells `parse` which type to produce. Without it, Rust can't know which number type you want.

---

### What expect prints on failure

If the string isn't a number, `expect` panics with your message, followed by the error:

```
the text should be a number: ParseIntError { kind: InvalidDigit }
```

The message says what the program expected, and the error says what it actually got. Compare that with `unwrap`, which would only report the error. A
message that explains the expectation is easier to act on, which is why production code uses `expect` in most places.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
The number is 42
```

The comments in `src/main.rs` suggest changing `"42"` to `"forty-two"`. The program then panics with the message above.

---

*For more detail, see section 9.2 "Recoverable Errors with Result" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html).*
