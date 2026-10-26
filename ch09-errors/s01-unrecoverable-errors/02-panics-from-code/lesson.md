## Panics from Code

Some panics aren't requested by the code at all. They come from an operation that can't produce a valid result, such as reading past the end of a
vector. Rust stops the program, rather than returning garbage.

---

### Reading past the end

The sample has a vector of three elements, and indexes 0, 1, and 2 are valid. It reads index 99:

```rust,ignore
let v = vec![1, 2, 3];
println!("{}", v[99]);
```

Rust panics. Without a backtrace, the output is:

```
thread 'main' (55004) panicked at src\main.rs:5:21:
index out of bounds: the len is 3 but the index is 99
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The message tells you the length of the vector and the index that was requested, which are the two facts you need. The location is line 5, column 21, which
is the index expression. The process ID in brackets will differ on your machine.

This protects your program. In C, reading past the end of a data structure is undefined behavior, which can leak data stored nearby, and attackers can
use that to read memory they shouldn't. Rust stops instead.

---

### Finding the cause with a backtrace

A panic in code that calls another library points at that library's line, not yours. A backtrace lists every function that was running when the panic
happened, starting from the most recent. To read it, start from the top, and find the first line that mentions a file you wrote. That's usually where the
problem started.

The Output pane has a **Backtrace** checkbox. When it's ticked, each run sets `RUST_BACKTRACE` for that run, so the panic shows a backtrace. The same
output then looks like this, with the frames trimmed to the ones that matter:

```
thread 'main' (44008) panicked at src\main.rs:5:21:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0: std::panicking::panic_handler
   ...
   5: panics_from_code::main
             at .\src\main.rs:5
   ...
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

The line that says `panics_from_code::main` at `src\main.rs:5` is the first frame that's yours, so it's where to look. The frames above it are the standard
library's code, and the frames below it are the code that called `main`. The exact frames depend on your Rust version.

Outside the runner, in a PowerShell terminal, you can set the variable yourself with `$env:RUST_BACKTRACE=1` before `cargo run`.

---

### Fixing the panic

With index panics, the fix is almost always to avoid asking for an index that doesn't exist. Chapter 8.1 showed `get`, which returns `None` rather than
panicking. Use it when an out-of-range index can happen in normal use. Use indexing when an out-of-range index means your code has a bug.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
thread 'main' (55004) panicked at src\main.rs:5:21:
index out of bounds: the len is 3 but the index is 99
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\panics_from_code.exe` (exit code: 101)
```

Tick **Backtrace** and run it again to see the stack as well. The comments in `src/main.rs` suggest changing the index to 2. The index is then valid, the
program prints `3`, and it doesn't panic.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
What is the name of the environment variable you should set to a value other than `0` to see the backtrace of a panic?

○ `RUST_PANIC`
→ Not quite. The variable is named for backtraces, not for panics.
○ `RUST_DEBUG`
→ Not quite. Debug information is controlled by the build, not by this variable.
○ `RUST_LOG`
→ Not quite. That variable is used by logging crates, not by Rust's panic output.
◉ `RUST_BACKTRACE`
→ Yes. For example, you can run `RUST_BACKTRACE=1 cargo run` from a command line.
````

---

*For more detail, see section 9.1 "Unrecoverable Errors with panic!" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html).*
