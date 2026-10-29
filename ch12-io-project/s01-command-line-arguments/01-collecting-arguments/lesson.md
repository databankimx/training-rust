## Collecting the Command Line Arguments

Our project is `minigrep`, a simplified version of the `grep` search tool. The first step is to make it accept the arguments it needs: a string to search for,
and a file to search in. The program runs like this:

```console
cargo run -- searchstring example-filename.txt
```

The two hyphens tell `cargo` that the arguments after them are for the program, not for `cargo` itself.

---

### Reading the arguments

The function `std::env::args` returns an iterator of the arguments the program was started with. An iterator produces a series of values. `collect` turns it into
a collection, and here that's a `Vec<String>`:

```rust,ignore
use std::env;

let args: Vec<String> = env::args().collect();
println!("{args:?}");
```

The `use std::env;` line brings in the module rather than the function. That way, other functions from `std::env` are easy to reach, and `args` isn't mistaken for a
function defined in the current module. Chapter 7 covered this style of `use`.

Rust needs the type annotation `Vec<String>` here. `collect` can build many kinds of collection, so Rust needs to be told which one you want.

---

### The program name

The first item in the vector is the path of the program itself, not an argument you gave. It's the same as `argv[0]` in C. Its exact text depends on your computer,
so the output starts with something like `"target\\debug\\minigrep.exe"`, and the backslashes are doubled because `Debug` output escapes them.

Each argument you add comes after that. Your output looks like this, where the first item is the program path, shown here as `...`:

```
["...", "test", "sample.txt"]
```

With no arguments, the vector has only the program path.

---

### Try it in the runner

Choose a quick pick, or type in the arguments box. Then press **Go**.

* The quick pick **cargo run** runs the program with no arguments. The output shows only the program path.
* The quick pick **cargo run -- test sample.txt** runs it with two arguments. The output shows the path, followed by `test` and `sample.txt`.

The quick picks include the `--` separator, but the runner adds its own, so the program receives only the two arguments.

---

*For more detail, see section 12.1 "Accepting Command Line Arguments" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html).*
