## Saving the Argument Values

Now the program needs the two arguments in variables, so the rest of the program can use them. The first argument is the string to search for, and the
second is the file path.

```rust,ignore
let args: Vec<String> = env::args().collect();

let query = &args[1];
let file_path = &args[2];
```

Index 0 of the vector is the program path, as the last step showed, so the arguments start at index 1. `query` refers to the first argument, and `file_path` refers
to the second. Both are references into `args`, so they don't copy the strings.

The sample prints the two values, to show that they're in the right variables:

```
Searching for test
In file sample.txt
```

---

### What happens with no arguments

The program depends on having two arguments. Run it with none, and `args` has only one item. Indexing `args[1]` then panics, because that index doesn't exist:

```
index out of bounds: the len is 1 but the index is 1
```

The book deals with this in a later section, by checking the arguments and printing a helpful message. For now, the panic shows what the program expects.

---

### Try it in the runner

Choose a quick pick, or type in the arguments box, and press **Go**.

* **cargo run -- test sample.txt** prints the two lines above.
* **cargo run** panics with the index error. The message has the same form as the one above, which tells you the vector had one item.

Use the box for any other pair of values. The first value becomes `query`, and the second becomes `file_path`.

---

*For more detail, see section 12.1 "Accepting Command Line Arguments" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html).*
