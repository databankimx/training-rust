## Unrecoverable Errors with panic!

Some errors can't be recovered from. Rust handles them with the `panic!` macro. A panic prints a message, unwinds the stack by cleaning up each
function that was running, and then the program exits.

There are two ways a panic happens: an action in your code causes one, such as reading past the end of a vector, or your code calls `panic!` directly.
Both stop the program in the same way.

---

### Calling panic! directly

The sample calls `panic!` with a message:

```rust,ignore
fn main() {
    println!("Before the panic");

    // panic! stops the program with this message. Nothing after it runs.
    panic!("crash and burn");

    // Try: move the println! below the panic! line. Does it print? Why, or why not?
}
```

The first line runs as normal. The `panic!` stops the program, and nothing after it runs. The output is:

```
Before the panic

thread 'main' (6168) panicked at src\main.rs:5:5:
crash and burn
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The panic message goes to standard error, and the runner shows it in the Output pane. It gives the message, and the file, line, and column where the
panic happened. Here that's line 5, column 5, which is the `panic!` call itself. The number in brackets is the process ID, so it will differ on your
machine. The final line of the output is cargo's report that the program exited with code 101, which is the code Rust uses for a panic.

If the panic happens inside code that another crate wrote, the location points at that code, rather than the line of yours that caused it. The next step
covers how to track it back.

---

### Stopping, and unwinding

By default, a panic unwinds the stack: Rust walks back up through each function that's running, and cleans up its data. The other option is to abort,
which stops immediately and leaves the cleanup to the operating system. Aborting makes the binary smaller, and you can choose it in `Cargo.toml` with
`panic = 'abort'` in a `[profile]` section. For most programs, the default is right.

---

### When to panic

A panic is for a state the program can't continue from. It isn't a way to report an ordinary failure to the code that called you. The next sub-chapter
shows how to report errors that a caller can handle, and Chapter 9.3 covers when a panic is the right choice.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Before the panic

thread 'main' (6168) panicked at src\main.rs:5:5:
crash and burn
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\panic_macro.exe` (exit code: 101)
```

The process ID and the path to the executable depend on your machine. The comments in `src/main.rs` suggest moving the `println!` below the `panic!`. Rust
warns that the line can't be reached, and it doesn't print.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Which of the following is NOT a good reason to use a panic?

○ The program is about to perform a dangerous operation
→ Not quite. Stopping before a dangerous operation is a good reason to panic.
○ The program should stop executing as soon as possible
→ Not quite. Stopping at once is what a panic does, so it's a fair reason.
○ The program has reached an unrecoverable error state
→ Not quite. That's exactly the situation a panic is for.
◉ The program has reached an error state which should be communicated to a caller function
→ Yes. A panic doesn't report an error to a caller. Callers don't normally catch panics, so a recoverable error should be returned as a value instead.
````

---

*For more detail, see section 9.1 "Unrecoverable Errors with panic!" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html).*
