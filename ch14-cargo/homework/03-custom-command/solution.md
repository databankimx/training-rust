## Solution: A Custom Command

This walkthrough explains the example solution in `src/main.rs`. Read it after you've written your own version.

### Run the solution

Press **Run** to see the usage message. The runner passes no file names, so the program prints this to standard error, and exits with code 1:

```
usage: cargo lines <file>...
```

Press **Test** for the two unit tests.

To try the command, install it and give it files, in a terminal:

```console
cargo install --path .
cargo lines poem.txt other.txt
```

The output is one count per file, and then a total, such as `poem.txt: 9` and `total: 13`. The counts depend on the files you give it.

### How it works

Cargo passes the program's path first, and then the subcommand name, `lines`. `skip(2)` leaves out both, so `files` holds only the names typed after the command.

With no files, the program prints the usage message to standard error, and exits with `process::exit(1)`. Errors go to standard error so that the results can be redirected
without losing them, as Chapter 12 described.

Each file is read in turn. A file that can't be read is reported, and the loop goes on, so one missing file doesn't stop the others. `failed` records that something went
wrong, and the program exits with code 1 at the end. The total is printed only when there's more than one file, because a total of one file repeats its count.

The counting itself is in `count_lines`, a small function that the unit tests can check without any files.

### The extensions

A `--words` mode needs a way to tell the modes apart. The usual way is to check whether the first argument after `lines` is a flag, such as `--words`, and remove it
before the file names are read.

Counting every `.rs` file in a folder uses `fs::read_dir`, which returns the entries of a folder. The folder can come from an argument, or from the current directory,
which `env::current_dir` returns.

---

*This walkthrough accompanies the exercise in Chapter 14 of the course. It's not from the book.*
