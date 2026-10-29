## Homework: A Custom Command

**The problem:** Write a custom Cargo command, `cargo-lines`, that counts the lines in each file it's given, and prints the total when there's more than one file.

Work in VS Code. Create a project with `cargo new cargo-lines`, so the binary is named `cargo-lines`. Then:

* Read the file names from the command line. Remember that Cargo passes the program path, then the subcommand name `lines`, before your arguments.
* For each file, print its line count, as `file: count`. If a file can't be read, print an error to standard error, and carry on with the others.
* If you gave more than one file, print the total at the end, as `total: count`.
* If you gave no files, print the usage message `usage: cargo lines <file>...` to standard error, and exit with code 1.
* Write unit tests for the function that counts lines. Check a text with three lines, and an empty text.

When the program is finished, install it from its folder with `cargo install --path .`, and run it from anywhere.

---

### Hints

* `env::args().skip(2)` gives the file names. The first two items are the program path and `lines`.
* `text.lines().count()` counts lines. An empty text has no lines.
* To stop with an error code, use `std::process::exit(1)`.

---

### Check your work

In a terminal, from a folder with a couple of text files:

```console
cargo lines poem.txt other.txt
```

It should print one count per file, and a total. Run `cargo lines` with no files. It should print the usage message, and the command should exit with code 1.

---

### Extensions

* Add a `--words` mode that counts words instead of lines. How would you tell the modes apart in the arguments?
* Count the lines in every `.rs` file in a folder, using `std::fs::read_dir`. Where does the folder name come from?

---

*This exercise is not from the book. It practices custom Cargo commands from Chapter 14.*
