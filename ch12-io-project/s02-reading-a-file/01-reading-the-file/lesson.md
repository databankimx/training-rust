## Reading a File

The program now has the query and the file path. The next task is to read the file, so the program can search it. Rust's standard library has `std::fs::read_to_string`,
which reads a whole file into a `String`:

```rust,ignore
use std::fs;

let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
```

The function returns a `Result`. Chapter 9 covered how to handle one, and this program uses `expect`. If the file can't be read, `expect` stops the program with
the message it's given, followed by the reason. A missing file is a common cause, so the program can't search a file it can't open.

The sample prints the search terms, then the file's contents:

```
Searching for test
In file poem.txt
With text:
I'm nobody! Who are you?
Are you nobody, too?
...
```

The file is `poem.txt`, which sits in the same folder as `Cargo.toml`. `cargo run` runs the program from that folder, so a file name with no folder part is found there.

---

### Try it in the runner

Choose a quick pick, or type in the arguments box, and press **Go**.

* **cargo run -- test poem.txt** prints the search terms, followed by the whole poem.
* **cargo run -- nobody poem.txt** does the same, with a different query. The query doesn't change the output yet, because the program doesn't search until a later
  section.
* **cargo run -- test missing.txt** panics with the `expect` message, followed by the reason the file couldn't be read. The reason is different on each operating system,
  so yours may read differently from the book.

---

*For more detail, see section 12.2 "Reading a File" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-02-reading-a-file.html).*
