## Unit Tests

Tests fall into two broad kinds. *Unit tests* are small, and check one module in isolation. They sit in the same file as the code they test, and they can check
private functions. *Integration tests* are outside the library, and use only its public interface, so they check how several parts work together.

Unit tests belong in the `src` file they test, inside a module named `tests` marked `#[cfg(test)]`. The `cfg(test)` attribute means the module is compiled only for
`cargo test`, so it's left out of the program and the library, which keeps them smaller.

---

### The sample

The sample is a library crate named `adder`. It has a public function, `add_two`, which calls a private function, `internal_adder`:

```rust,ignore
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(left: i32, right: i32) -> i32 {
    left + right
}
```

The unit test lives in the same file, and can call the private function. The `use super::*;` line brings the parent module's items into the `tests` module:

```rust,ignore
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

Tests are ordinary Rust code, and a child module can use private items of its parent. That's why this test can reach `internal_adder`, even though code outside the file
can't.

---

### Running the program and the tests

**Run** prints the program's output. The program is in `src/main.rs`, and it calls the library's public function:

```
5
```

**Test** runs the unit test, and its line shows the test's name as `tests::internal`:

```
test tests::internal ... ok
```

The output also includes a section for the binary, which has no tests. Its summary says `running 0 tests`. Each part of the package is reported in its own section.

---

### Whether to test private functions

Some people argue that only public behavior should be tested. Rust doesn't force either view. Its privacy rules let you test private functions directly, and you can
choose not to. The choice is yours.

---

### Try it in the runner

Press **Test**. The output should include `test tests::internal ... ok`.

The comments in `src/lib.rs` suggest adding a test for `add_two(0)`. Ask yourself whether the new test needs to be inside `tests`, or whether it could sit elsewhere.

---

*For more detail, see chapter 11 "Test Organization" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch11-03-test-organization.html).*
