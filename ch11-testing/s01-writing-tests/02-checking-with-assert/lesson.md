## Checking Results with assert!

`assert!` is the simplest check. It takes one expression that evaluates to a `bool`. If the value is `true`, the test carries on. If it's `false`, the macro
panics, and the test fails.

It suits methods that return a `bool`, such as the `can_hold` method from Chapter 5.

---

### The sample

The sample is the `Rectangle` and `can_hold` code from Chapter 5. The tests check both directions of the comparison:

```rust,ignore
#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle { width: 8, height: 7 };
    let smaller = Rectangle { width: 5, height: 1 };
    assert!(larger.can_hold(&smaller));
}

#[test]
fn smaller_cannot_hold_larger() {
    let larger = Rectangle { width: 8, height: 7 };
    let smaller = Rectangle { width: 5, height: 1 };
    assert!(!smaller.can_hold(&larger));
}
```

The second test negates the result, because the correct answer is `false`. Testing the case where the answer should be false matters as much as the other case. A
function that always returned `true` would pass the first test and fail the second.

The program's own output, from **Run**, is:

```
larger can hold smaller: true
```

The tests, from **Test**, report two passing tests:

```
running 2 tests
test tests::larger_can_hold_smaller ... ok
test tests::smaller_cannot_hold_larger ... ok

test result: ok. 2 passed; 0 failed; 0 ignored
```

---

### Catching a bug

Suppose someone changes `>` to `<` in the comparison of widths. The first test then fails, because `8 < 5` is `false`. The failure report names the test, and says
the assertion failed on the line where it was written. The second test would also catch the change, as a shrinking rectangle would now appear to hold a larger one.

---

### Try it in the runner

Press **Test**. The output should show two passing tests.

The comments in `src/main.rs` suggest changing `>` to `<` in `can_hold`. Then **Test** reports which test catches the bug.

---

*For more detail, see chapter 11 "Writing Automated Tests" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch11-01-writing-tests.html).*
