## Propagating Errors with ?

Often a function can't decide what to do about an error. The caller has more information, so the function should return the error, and let the
caller choose. Passing an error back up like this is called *propagating* it.

---

### A function that propagates

The sample reads a username from `hello.txt`. Both `File::open` and `read_to_string` can fail, and the function returns either error to its caller:

```rust,ignore
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

The `?` after a `Result` does what a `match` would do. If the value is `Ok`, the value inside is returned, and the function continues. If the value is `Err`,
the function returns that error straight away. You could write the same thing with a `match` and an early `return`, but `?` removes that boilerplate.

The `?` can be chained onto a call, as in `File::open("hello.txt")?.read_to_string(...)`, and that's a common way to shorten the code further.

---

### The sample

The sample writes `will` to `hello.txt`, then calls the function, and prints the result:

```rust,ignore
fs::write("hello.txt", "will").expect("could not write hello.txt");

match read_username_from_file() {
    Ok(name) => println!("username: {name}"),
    Err(error) => println!("could not read username: {:?}", error.kind()),
}
```

The output is:

```
username: will
```

The sample writes the file so it has something to read. Running it creates `hello.txt` in the step's folder, and the file stays there.

---

### Where ? can be used

The `?` returns early from the function, so the function's return type has to match what `?` gives back. A function that returns `Result` can use `?` on a
`Result`, and a function that returns `Option` can use `?` on an `Option`. You can't mix the two. Calling `?` on a `Result` in a function that returns
`Option` is a compile error, and the fix is to convert the value, for example with `ok` or `ok_or`.

A `main` function that returns nothing can't use `?` at all. The next step shows how to fix that.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
username: will
```

The comments in `src/main.rs` suggest deleting the `fs::write` line, and deleting `hello.txt` from the folder before running again. Without the file, the
`File::open` call fails, and the `Err` arm runs with `NotFound`.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Given an arbitrary expression `e` of type `Result<T, E>`, which code snippet best represents how `e?` is translated?

○ `if let Err(err) = e { return Err(err); }`
→ Not quite. That returns the error, but it doesn't give back the value inside `Ok`, so the expression has no result.
○ `e.unwrap()`
→ Not quite. `unwrap` panics on an error, rather than returning it to the caller.
○ `match e { Ok(x) => x, Err(err) => panic!("{err}") }`
→ Not quite. This panics on an error, while `?` returns the error to the caller.
◉ `match e { Ok(x) => x, Err(err) => { return Err(err); } }`
→ Yes. The `Ok` value is the result of the expression, and the `Err` is returned from the function.
````

````quiz
Consider this program, where `hello.txt` contains `will`.

```rust
fn read_username_from_file() -> Option<String> {
  let mut username_file = File::open("hello.txt")?;
  let mut username = String::new();
  username_file.read_to_string(&mut username)?;
  Some(username)
}

fn main() {
  println!("{}", read_username_from_file().unwrap());
}
```

Does this program compile? If not, on which line does Rust report the problem?

○ This program compiles
→ Not quite. `File::open` returns a `Result`, but this function returns an `Option`, so the `?` on line 2 doesn't fit.
◉ This program does not compile, on line 2
→ Yes. Line 2 uses `?` on a `Result`, in a function that returns an `Option`. The value must be converted first, for example with the `ok` method.
○ This program does not compile, on line 4
→ Not quite. Line 4 uses `?` on the result of `read_to_string`, which is a `Result`, so that error would be a different one.
````

---

*For more detail, see section 9.2 "Recoverable Errors with Result" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html).*
