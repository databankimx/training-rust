## Output Capture

By default, `cargo test` runs the tests in parallel, and captures anything they print to standard output. The output is hidden for a test that passes, so you see
only a short summary. For a test that fails, the captured output appears with the failure, next to the reason it failed. That keeps the results easy to read.

---

### The sample

The function `prints_and_returns_10` prints its parameter and returns 10. The two tests call it with different values:

```rust,ignore
#[test]
fn this_test_will_pass() {
    let value = prints_and_returns_10(4);
    assert_eq!(10, value);
}

#[test]
fn this_test_will_fail() {
    let value = prints_and_returns_10(8);
    assert_eq!(5, value);
}
```

**Run** prints the program's output, which is:

```
10
```

**Test** runs both tests. The passing test's `I got the value 4` line is captured, so it doesn't appear. The failing test's output is shown in a section of its own.
The output looks like this, with some lines shortened:

```
running 2 tests
test tests::this_test_will_fail ... FAILED
test tests::this_test_will_pass ... ok

failures:

---- tests::this_test_will_fail stdout ----
I got the value 8
thread 'tests::this_test_will_fail' panicked at src\main.rs:...:
assertion `left == right` failed
  left: 5
 right: 10

failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored
```

The line `I got the value 8` is in the failure section, because it was printed by the test that failed. The line for the passing test isn't shown.

---

### Showing the output of passing tests

If you want to see what passing tests print as well, run `cargo test -- --show-output` in a terminal. Then `I got the value 4` appears too. The runner's Test button
doesn't pass that option, so this flag is for use in a terminal.

---

### Try it in the runner

Press **Test**. The output should match the failure section above, with the passing test marked `ok`.

The comments in `src/main.rs` suggest making the passing test fail. Its printed line then appears in the failure section too, because it's now a failing test.

---

*For more detail, see chapter 11 "Controlling How Tests Are Run" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch11-02-running-tests.html).*
