## main Returns a Result

The `main` function is special, because it's where the program starts and ends. Until now, all of our `main` functions returned nothing, so `?` couldn't
be used in them. `main` is allowed to return a `Result`, and that makes `?` available at the top level.

---

### The sample

The sample writes a file, opens it, reads a username from it, and prints it. Each step can fail, so each one ends with `?`:

```rust,ignore
fn main() -> Result<(), Box<dyn Error>> {
    fs::write("hello.txt", "will")?;

    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    println!("username: {username}");

    Ok(())
}
```

The return type `Result<(), Box<dyn Error>>` means a success with no value, or any kind of error. `Box<dyn Error>` is a trait object, which Chapter 18
covers in detail. For now, read it as "any error". That's why `?` works on the `io::Error` values from each call. The output is:

```
username: will
```

The program ends with `Ok(())`, which means success.

---

### What happens on an error

When `main` returns `Err`, the program prints the error, and exits with a nonzero exit code. Successful programs exit with code `0`. Rust follows the same
convention as C, where any other value signals that something went wrong.

If you delete `hello.txt` and remove the `fs::write` line, the `File::open` call fails. The runner then shows an error starting with `Error:`, followed by the
error's `Debug` text. The exact text depends on your operating system, but the exit code is always nonzero.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
username: will

— Finished with exit code 0 —
```

The comments in `src/main.rs` explain the `Ok(())` at the end. Removing it is a compile error, because `main` must return a value of the declared type.

---

*For more detail, see section 9.2 "Recoverable Errors with Result" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html).*
