## Writing a Test

A test in Rust is an ordinary function, marked with the `#[test]` attribute. The body of a test usually does three things: sets up any data it needs, runs
the code under test, and checks that the result is what it should be.

The `cargo test` command finds every function marked `#[test]`, runs it, and reports whether it passed or failed.

---

### The sample

The sample has a small function, `add`, and a `main` that uses it. The tests live in a module marked `#[cfg(test)]`, which means the module is compiled only when
you run `cargo test`. The program itself never includes it:

```rust,ignore
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

`use super::*;` brings the functions of the outer module into the test module, so `add` can be called. `assert_eq!` checks that two values are equal, and
fails the test if they're not.

---

### Running the program and the tests

The runner has two buttons for this sample. **Run** runs the program, and prints:

```
2 + 2 = 4
```

**Test** runs `cargo test`. Among the Cargo messages, it prints:

```
running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored
```

The line `test tests::it_works ... ok` shows the test's full name, which includes its module. The summary `test result: ok` means every test passed. The
counts after it show how many passed, failed, or were ignored. Other summary fields, such as the number of tests filtered out, appear too, and we'll cover
them in the next sub-chapter.

---

### Try it in the runner

Press **Test**. Its output should include the lines above.

The comments in `src/main.rs` suggest changing the expected value to 5. The test then fails, and the output shows the left and right values, so you can see what
the code produced.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
What is the annotation you add to a function to indicate that it's a unit test?

[ #[test] ]
→ The `#[test]` attribute tells cargo test to run the function as a test, not treat it as library code.
````

---

*For more detail, see chapter 11 "Writing Automated Tests" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch11-01-writing-tests.html).*
