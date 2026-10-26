## When panic! Is the Right Choice

Returning a `Result` is the default when you write a function that can fail. The caller then decides whether to recover, or to treat the error as fatal.
Some situations call for a panic, though, and this step covers the common ones.

---

### Examples, prototypes, and tests

In an example, a robust error path can hide the point of the code. Calling `unwrap` or `expect` is understood as a placeholder for real error handling.
They also mark places to come back to when you're prototyping.

In a test, a failed call should fail the test. `panic!` is how a test is marked as failed, so `unwrap` and `expect` are the right tools there.

---

### When you know more than the compiler

Sometimes you can see that a `Result` is always `Ok`, even though the compiler can't. The sample parses a hardcoded IP address:

```rust,ignore
let home: IpAddr = "127.0.0.1".parse().expect("127.0.0.1 is a valid IP address");
println!("home address: {home}");
```

`parse` still returns a `Result`, because the compiler doesn't check the string's contents. Because you can see the address is valid, `expect` is
appropriate, and the message records the assumption. The output is:

```
home address: 127.0.0.1
```

If the address came from user input instead, it could fail, and the code would need to handle the `Err` value. The `expect` message also tells a
future reader to change the code if the address ever comes from somewhere else.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
home address: 127.0.0.1
```

The comments in `src/main.rs` suggest changing the address to text that isn't an address. The program then panics with your `expect` message, followed by
the parse error. That's the failure the message warns about: a broken assumption.

---

*For more detail, see section 9.3 "To panic! or Not to panic!" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html).*
