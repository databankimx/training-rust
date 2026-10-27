## Writing Errors to Standard Error

A command line program has two output streams. *Standard output* (`stdout`) carries the program's normal results. *Standard error* (`stderr`) carries error
messages. Most terminals show both, but a user can send them to different places.

For example, a user might save a search's results to a file, and still want to see any error on the screen:

```console
cargo run -- to poem.txt > output.txt
```

The `>` sends standard output to `output.txt`. Standard error isn't redirected, so an error would still appear on the screen. That only works if the program writes
its errors to standard error. If it wrote them with `println!`, the error would end up in the file, and the user would see nothing.

---

### eprintln!

The `println!` macro writes to standard output. The `eprintln!` macro takes the same arguments, and writes to standard error:

```rust,ignore
eprintln!("Problem parsing arguments: {err}");
```

`minigrep` uses `eprintln!` for both of its error messages, in `main`, and uses `println!` for the matching lines. The results go to standard output, and the errors
go to standard error. That's why the two kinds of output can be separated.

The runner's Output pane shows both streams together, but it styles standard error differently, so an error stands out from the results. Try the commands below
and compare the colors.

---

### Try it in a terminal

The runner can't redirect output, so these commands run in a terminal, from this step's folder:

```console
cargo run > output.txt
```

No arguments cause an error. The message appears on the screen, and `output.txt` is empty.

```console
cargo run -- to poem.txt > output.txt
```

Nothing appears on the screen, and `output.txt` contains the two matching lines.

---

### Try it in the runner

Choose a quick pick, and press **Go**.

* **cargo run** prints the error in the styled error color, and exits with code 1.
* **cargo run -- to poem.txt** prints the matching lines in normal color.

---

*For more detail, see section 12.6 "Writing Error Messages to Standard Error Instead of Standard Output" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-06-writing-to-stderr-instead-of-stdout.html).*
