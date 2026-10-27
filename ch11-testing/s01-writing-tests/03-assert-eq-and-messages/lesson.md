## Testing Equality with assert_eq! and Custom Messages

Many tests check that a result equals an expected value. You could write that with `assert!` and `==`, but `assert_eq!` is clearer, and it reports more when it
fails. It prints both values, so you can see what the code produced without rerunning it.

`assert_ne!` does the opposite: it passes when the two values differ. It's useful when you don't know the exact result, but you know what it must not be.

---

### The sample

The sample has an `add_two` function, and the test checks that `add_two(2)` is `4`:

```rust,ignore
#[test]
fn it_adds_two() {
    assert_eq!(add_two(2), 4);
}
```

The second test checks a greeting. The exact wording might change, so the test checks only that the name appears in the result. It also adds a custom message
that includes the value produced:

```rust,ignore
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain the name, the value was `{result}`"
    );
}
```

Custom messages come after the required arguments, and they accept the same format string as `println!`. They're worth adding when the reason for the check isn't
obvious from the assertion alone.

The program's output, from **Run**, is:

```
4
Hello Carol!
```

The tests, from **Test**, report two passing tests:

```
running 2 tests
test tests::it_adds_two ... ok
test tests::greeting_contains_name ... ok

test result: ok. 2 passed; 0 failed; 0 ignored
```

---

### What a failure looks like

If `add_two` added 3 instead of 2, the first test would fail with a message like this:

```
assertion `left == right` failed
  left: 5
 right: 4
```

The `left` value is what the code produced, and the `right` value is what was expected. In Rust, the order of the two arguments doesn't affect the result, so
`assert_eq!(4, add_two(2))` reports the same way.

`assert_eq!` needs both values to implement `PartialEq` and `Debug`. Most standard library types do. For your own types, derive both traits.

---

### Try it in the runner

Press **Test**. The output should show two passing tests.

The comments in `src/main.rs` suggest changing `add_two` to add 3. The test then fails, and the output shows the left and right values.

---

*For more detail, see chapter 11 "Writing Automated Tests" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch11-01-writing-tests.html).*
