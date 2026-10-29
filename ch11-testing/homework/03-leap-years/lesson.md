## Homework: Leap Years

**The problem:** A function that decides whether a year is a leap year has a bug. Your job is to find the bug with tests, and fix it.

The buggy function is:

```rust
fn is_leap_year(year: u32) -> bool {
    year % 4 == 0
}
```

The rules are: a year is a leap year if it's divisible by 4. The exception is a century year, one divisible by 100, which is a leap year only if it's also
divisible by 400. So 2024 and 2000 are leap years, but 2023 and 1900 are not.

Create a project in VS Code, with `cargo new leap_years`. Put the function above in `src/main.rs`, and then do the following:

1. Write tests for the rules. At least one should check a century that isn't a leap year, such as 1900, and at least one should check a century that is, such as 2000.
2. Run `cargo test`. The century test should fail, which shows the bug.
3. Fix the function so that every test passes.

---

### Hints

* Think about what `%` returns. `year % 100 == 0` is true for every century year.
* The rule has two parts, so the fix needs both conditions. Write the two conditions in the right order, using `&&` and `||`.
* Test the boundary years too: the years just before and after a century, and a century divisible by 400.

---

### Check your work

Once your fix is in, your `main` should print these lines for the years 2024, 1900, 2000, and 2023:

```
2024: true
1900: false
2000: true
2023: false
```

Your tests should all pass, and the century test should have failed before the fix.

---

### Extensions

* Add a function that returns the number of days in a February, using `is_leap_year`. Test it for a leap year, a common year, and a century year.
* Write a test with years that are not valid, such as 0. Should `is_leap_year` accept them? What would a good answer be?
* Change `year` from `u32` to `i32`. Which tests need to change, if any?

---

*This exercise is not from the book. It practices writing tests that find bugs, from Chapter 11.*
