## Options and Filters

The runner's Test button runs every test with the default settings. A terminal gives you more control. Options placed before `--` go to `cargo test`, and
options placed after `--` go to the test program itself. The runner doesn't pass these options, so the commands below are for a terminal, run from the step's
folder.

---

### The sample

The sample is the `add_two` function, with three tests. Their names are the same as the book's Listing 11-11, and you'll use them to choose which tests run:

```rust,ignore
#[test]
fn add_two_and_two() {
    assert_eq!(4, add_two(2));
}

#[test]
fn add_three_and_two() {
    assert_eq!(5, add_two(3));
}

#[test]
fn one_hundred() {
    assert_eq!(102, add_two(100));
}
```

**Run** prints the program's output, which is:

```
4
```

**Test** runs all three tests. They run in parallel, so the lines can appear in any order:

```
running 3 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok
test tests::one_hundred ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

### Running tests in parallel, or one at a time

By default, tests run in parallel threads. That's quick, but it's only safe when the tests don't share state. Two tests that write the same file, for example, can
interfere with each other. Running the tests one at a time avoids that:

```console
cargo test -- --test-threads=1
```

This runs the tests on one thread. It takes longer, but the tests can't interfere with each other. If a test fails only when run in parallel, a shared file or
environment variable is a likely cause.

---

### Running a single test by name

Pass the name of a test to `cargo test`, and only that test runs:

```console
cargo test one_hundred
```

The summary then shows `2 filtered out`, because the other two tests didn't match. Only the first name you give is used, so you can't name several tests this way.

To run several tests, pass part of their names. Any test whose name contains the text runs:

```console
cargo test add
```

This runs `add_two_and_two` and `add_three_and_two`, and filters out `one_hundred`. A module's name is part of each test's name, so `cargo test tests` runs every test in
the module.

---

### Running ignored tests

As the last step showed, `cargo test -- --ignored` runs only the tests marked `#[ignore]`. `cargo test -- --include-ignored` runs every test.

---

### Try it in the runner, and then in a terminal

Press **Test** first, and check that all three tests pass. Then type `add` into the box below the output, or pick one of the quick picks, and press Enter. The
box gives the arguments to `cargo test`, so `add` runs the two tests whose names contain it. Try `one_hundred` next. You can also try the same commands in a
terminal, from this step's folder:

```console
cargo test add
cargo test one_hundred
```

The first runs two tests, and the second runs one.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
When running `cargo test` with no additional configuration, which of the following actions may not work correctly if done by multiple tests?

○ Logging strings to stdout
→ Not quite. Captured output is kept separate for each test, so printing is safe.
○ Failing via panics
→ Not quite. A panic in one test fails only that test.
○ Reading data from a single database
→ Not quite. Reading is safe. Only changing shared state needs care.
◉ Writing text to a single file
→ Yes. Tests run at the same time, so two tests can overwrite each other's writes, and one can read the other's data.
````

````quiz
Consider a program with the following unit tests:

```rust
#[test]
fn test_the_logger() { /* ... */ }

#[test]
fn test_the_database() { /* ... */ }

#[test]
fn test_logger_and_database() { /* ... */ }
```

What is the shortest string you can pass to `cargo test <the_string>` such that only `test_the_logger` and `test_the_database` are executed?

[ h ]
→ Yes. The letter `h` appears in "the" in the first two names, but the third name, `test_logger_and_database`, has no `h` at all. The letter `h` is the shortest choice here.
````

---

*For more detail, see chapter 11 "Controlling How Tests Are Run" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch11-02-running-tests.html).*
