## Matching on Different Errors

The last step handled every error the same way. Often, different errors call for different actions. A missing file can be created, while a file that
can't be read because of permissions should stop the program.

---

### Two levels of match

The error from `File::open` has a method, `kind`, which returns an `ErrorKind`. That enum lists the reasons an input or output operation can fail. A
nested `match` picks out the reason you care about:

```rust,ignore
match file_result {
    Ok(_file) => println!("missing.txt opened"),
    Err(error) => match error.kind() {
        ErrorKind::NotFound => println!("missing.txt does not exist yet"),
        other => panic!("could not open missing.txt: {other:?}"),
    },
}
```

The outer `match` separates success from failure. The inner `match` handles the failure, and it has two arms. `NotFound` is expected, so the program
carries on. `other` catches every remaining reason, and the program panics with the reason in the message.

The sample prints:

```
missing.txt does not exist yet
```

The book's version of this example creates the missing file, and returns a handle to it. The sample here doesn't create a file, so you can run it
repeatedly with the same result.

---

### Why the catch-all panics

The inner `match` has to handle every kind of error, and the `other` arm does that. Panicking is a choice here: a reason the program doesn't expect, such
as lacking permission, means the program can't do its job, so stopping is the right response. Chapter 9.3 covers when a panic is appropriate.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
missing.txt does not exist yet
```

The comments in `src/main.rs` suggest creating a file named `missing.txt` in the project folder, and running again. The `Ok` arm then runs, and prints `missing.txt opened`.

---

*For more detail, see section 9.2 "Recoverable Errors with Result" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html).*
