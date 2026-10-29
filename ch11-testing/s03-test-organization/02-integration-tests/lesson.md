## Integration Tests

Integration tests live outside the library, in a `tests` folder next to `src`. Each file there is compiled as its own crate, and it can use only the public
interface of the library. They check that several parts of the library work together, and they test the code the way outside code will use it.

Cargo knows to look in `tests`, so you don't need `#[cfg(test)]` on them. They run only with `cargo test`.

---

### The sample

The sample is the same `adder` library as the last step, with a public `add_two`. The integration test is in `tests/integration_test.rs`. The Source tab lists only
the files in `src`, so the file's contents are shown here:

```rust,ignore
use adder::add_two;

#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(2));
}
```

The file brings the library into scope with `use adder::add_two;`, because it's a separate crate. It can't see `internal_adder`, which is private, so the test
uses only the public function.

---

### Running the program and the tests

**Run** prints the program's output, which is:

```
5
```

**Test** runs the unit test, the integration test, and the documentation tests, in three sections. Each section has its own summary. Roughly, the output looks
like this:

```
Running unittests src\lib.rs
test tests::internal ... ok

Running unittests src\main.rs
running 0 tests

Running tests\integration_test.rs
test it_adds_two ... ok

Doc-tests adder
running 0 tests
```

The integration section starts with the line `Running tests\integration_test.rs`, and lists each of its test functions. Each file in `tests` gets its own section.
If a unit test fails, the later sections don't run, so a failing unit test stops the integration and documentation tests from running.

---

### Running one test file

To run only the tests in one integration file, pass `--test` followed by the file name, in a terminal:

```console
cargo test --test integration_test
```

The runner's **Test** button runs everything, so this command is for use outside the runner.

---

### Try it in the runner

Press **Test**. The output should show the three sections above, with the integration test marked `ok`.

The comments below are not in the source, so try this yourself: change `add_two(2)` in the integration test to `add_two(3)`. Which section fails, and do the later
sections run?

---

*For more detail, see chapter 11 "Test Organization" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch11-03-test-organization.html).*
