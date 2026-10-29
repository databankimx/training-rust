## Ignored Tests

Some tests are slow, and you may not want them in every run. The `#[ignore]` attribute leaves a test out of an ordinary `cargo test`, while keeping it in the
code. Ignored tests are reported in the summary, so they're never forgotten.

---

### The sample

The sample has a quick test, and a test marked `#[ignore]`. The second test stands for one that takes minutes:

```rust,ignore
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    assert_eq!(1 + 1, 2);
}
```

**Run** prints the program's output, which is:

```
program ran
```

**Test** runs the tests that aren't ignored, and reports the ignored one in the summary:

```
running 2 tests
test tests::expensive_test ... ignored
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 1 ignored
```

The `ignored` label shows that `expensive_test` exists, but wasn't run.

---

### Running only the ignored tests

To run only the ignored tests, use `cargo test -- --ignored` in a terminal. To run every test, ignored or not, use `cargo test -- --include-ignored`. The runner's
Test button doesn't pass either option, so those commands are for use outside the runner.

Ignoring is a way to keep ordinary runs fast, not a way to hide failures. Run the ignored tests from time to time, so that a slow test still checks something.

---

### Try it in the runner

Press **Test**. The output should show `expensive_test` as ignored.

The comments in `src/main.rs` suggest removing `#[ignore]`. Both tests then run, and the summary reports two passing tests with no ignored ones.

---

*For more detail, see chapter 11 "Controlling How Tests Are Run" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch11-02-running-tests.html).*
