## Shared Helpers and Binary Crates

Two details matter once a project has several test files: where shared helper code goes, and what happens in a project that's only a binary.

---

### The sample

The sample has the `adder` library from the last step, and an integration test that needs a helper. The helper lives in `tests/common/mod.rs`:

```rust,ignore
// tests/common/mod.rs
pub fn setup() -> &'static str {
    "setup done"
}
```

The integration test loads the helper with `mod common;`, and calls it:

```rust,ignore
// tests/integration_test.rs
use adder::add_two;

mod common;

#[test]
fn it_adds_two() {
    assert_eq!(common::setup(), "setup done");
    assert_eq!(4, add_two(2));
}
```

**Run** prints the program's output, which is:

```
5
```

**Test** runs the tests, and reports one section for each test crate. The output has a unit test section, an integration test section, and a documentation test section.
The `common` module has no section of its own, which is the point of the subdirectory:

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

If the helper were in `tests/common.rs` instead, the output would contain a `Running tests\common.rs` section with `running 0 tests`. That's the unwanted section the subdirectory avoids.

---

### Why the helper needs a subdirectory

Each file in `tests` is its own crate, so a helper in a file directly in `tests` would be treated as one. Putting it in `tests/common/mod.rs` avoids that. The `mod common;` line
then loads it as a module of the test crate, using the module syntax from Chapter 7 and the older `mod.rs` naming.

---

### Binary crates

A project with only `src/main.rs` can't have integration tests that use its functions. Integration tests can reach only library crates, because a binary is meant to
run on its own.

For that reason, Rust projects that provide a binary usually keep their logic in `src/lib.rs`, and make `src/main.rs` a small wrapper around it. This sample does that:
`main` only calls `adder::add_two`, and the library holds the tested logic.

---

### Try it in the runner

Press **Test**. The output should match the sections above, with no section for `common`.

The comments in `tests/integration_test.rs` suggest moving `setup` into `tests/common.rs`. Then a new `Running tests\common.rs` section appears, with no tests.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Which of the following is NOT a good reason to wrap unit tests in `#[cfg(test)] mod tests { ... }`?

○ It can reduce the size of generated compiler artifacts
→ Not quite. Leaving the tests out of the build keeps the compiled output smaller, so this is a good reason.
○ It can improve compile times
→ Not quite. When you only build the library, the test code is skipped, which saves time.
○ It separates test helper functions from library code
→ Not quite. Helpers in the module are kept apart from the code that ships.
◉ It gives your tests access to private functions
→ Yes. Every unit test in a file can reach that file's private functions, whether it's in a `tests` module or not. The module is about keeping test code out of the build.
````

---

*For more detail, see chapter 11 "Test Organization" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch11-03-test-organization.html).*
