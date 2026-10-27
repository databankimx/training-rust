## Homework: Temperature

**The problem:** Write a function that converts a temperature from Celsius to Fahrenheit. The formula is `F = C * 9 / 5 + 32`. A temperature below absolute zero,
-273.15 Celsius, is impossible, so the function returns an error for it.

Create a project in VS Code, as described in the chapter overview. Then write:

* `celsius_to_fahrenheit(c: f64) -> Result<f64, String>`, which returns the Fahrenheit value, or an `Err` with a message.
* Tests, in a `tests` module marked `#[cfg(test)]`, that check:
  * 0 Celsius is 32 Fahrenheit.
  * 100 Celsius is 212 Fahrenheit.
  * -40 is the same in both scales.
  * A temperature below absolute zero gives an error.

Write the tests before the function, if you like. Each test should fail until the function is written, and then pass.

---

### Hints

* Use `assert_eq!` for the three normal cases. `Result<f64, String>` implements `PartialEq` and `Debug`, so `assert_eq!` can compare it.
* For the error case, `assert!(result.is_err())` checks that the function returned an error, without caring what the message says.
* Once your tests pass, add a `main` that prints a few conversions. Running `cargo run` shows the program's output, and `cargo test` shows the tests.

---

### Check your work

Your program's `main` should print something like this, if it converts the values 0, 100, -40, and -300:

```
0.0 C -> Ok(32.0)
100.0 C -> Ok(212.0)
-40.0 C -> Ok(-40.0)
-300.0 C -> Err("-300 C is below absolute zero")
```

All four tests should pass when you run `cargo test`.

---

### Extensions

* Add a function for the reverse conversion, from Fahrenheit to Celsius, and test it. Which of its cases can be an error?
* Make the error type your own enum, with one variant for each kind of error. How do the tests change?
* Add a test that checks a value just above absolute zero, such as -273.14. What does it check that the other tests don't?

---

*This exercise is not from the book. It practices tests and `Result` from Chapter 11.*
