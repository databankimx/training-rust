## Homework: Several Files

**The problem:** Let a user search more than one file at once, such as `cargo run -- body poem.txt other.txt`. Each matching line is printed with the name of its
file. If one file can't be read, report it on standard error, and still search the others. The program exits with code 1 only if some file failed.

Work in your own `minigrep` project, from VS Code. Then:

* Change `Config` so it holds `file_paths: Vec<String>` instead of one `file_path`. Every argument after the query is a file path.
* Keep the check for too few arguments. The query and at least one file path are required.
* Change `run` so that it searches each file in turn. Print each match as `file: line`.
* When a file can't be read, print `Could not read file: reason` on standard error, and move on to the next file.
* If any file couldn't be read, `run` should return an error at the end. Otherwise it returns `Ok`.

Create a second file for testing, such as `other.txt`, with some lines of your own. Include one line that matches `body`, and one that doesn't.

---

### Hints

* `args[2..]` is a slice of every argument from index 2 on. `to_vec()` or `iter().cloned().collect()` turns it into a `Vec<String>`.
* A `for` loop over `&config.file_paths` borrows each path, so `config` can still be used afterwards.
* `match` on `fs::read_to_string(path)`. The `Ok` arm searches. The `Err` arm prints to standard error and records that something failed.
* To return an error from a `Result<(), Box<dyn Error>>` with a message, `Err("some files could not be read".into())` works.

---

### Check your work

With a `poem.txt` and an `other.txt` that both contain `body`, and with a missing `missing.txt`:

```console
cargo run -- body poem.txt other.txt
cargo run -- body poem.txt missing.txt other.txt
```

The first command prints each match with its file name. The second prints the matches from `poem.txt` and `other.txt`, and an error for `missing.txt` on standard
error. It exits with code 1.

---

### Extensions

* Print the file name only when there is more than one file. How would you check that?
* Read the files in parallel with threads. What would need to change for the output to stay in order? (Chapter 16 covers threads.)

---

*This exercise is not from the book. It extends the minigrep project from Chapter 12.*
