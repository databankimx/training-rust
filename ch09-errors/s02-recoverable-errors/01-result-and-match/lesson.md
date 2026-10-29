## Recoverable Errors with Result

Most errors aren't serious enough to stop the whole program. If a file doesn't exist, the program may want to create it, rather than end. Rust
handles these errors with the `Result` enum, which lets a function say either that it succeeded, and give a value, or that it failed, and say why.

The enum is defined in the standard library like this:

```rust,ignore
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` is the type of the value returned on success, and `E` is the type of the error. Because both are generic, the same enum works for any pair of
success and error types. Chapter 10 covers generics in detail.

---

### Opening a file

`File::open` returns a `Result`. Its success type is `File`, and its error type is `io::Error`. The sample opens `hello.txt` and matches on the result:

```rust,ignore
let greeting_file_result = File::open("hello.txt");

match greeting_file_result {
    Ok(_file) => println!("hello.txt opened"),
    Err(error) => println!("could not open hello.txt: {:?}", error.kind()),
}
```

If the file isn't there, the result is `Err`, and `error.kind()` gives the reason as an `ErrorKind`. For a missing file, the kind is `NotFound`. The
sample's output in the project folder, where no `hello.txt` exists, is:

```
could not open hello.txt: NotFound
```

The `Ok` arm gets the file, which the sample ignores, using `_file`. The `Err` arm handles the failure. The program keeps running either way. Compare
this with `panic!` in the last sub-chapter: here the caller decides what happens.

---

### Why Result and not Option

`Option` can only say that a value is missing. `Result` can say why an operation failed. Opening a file can fail for many reasons, such as a missing file,
or a file you have no permission to read, so the error needs to carry a reason. That's why `File::open` returns a `Result`.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
could not open hello.txt: NotFound
```

The comments in `src/main.rs` suggest creating a `hello.txt` file in the project folder. The next run opens the file, and prints `hello.txt opened`.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Which of these statements best describes why `File::open` returns a `Result` and not an `Option`?

○ Because `Result` represents the possibility of failure, while `Option` cannot represent failures
→ Not quite. `Option` can represent that something is missing, so it can represent a failure too. The problem is that it can't say why.
○ Because `Result` uses fewer bytes at runtime than `Option` to represent failures
→ Not quite. The choice is about meaning, not the size of the type.
○ Because `Result` represents errors the same way as the underlying system calls
→ Not quite. The reason is what the caller needs, not how the operating system reports it.
◉ Because `Result` can represent why an operation failed, and file opening can fail for many reasons
→ Yes. `Option` can only say that a value is absent. `Result` carries the reason, which matters when a file can fail in several ways.
````

---

*For more detail, see section 9.2 "Recoverable Errors with Result" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html).*
