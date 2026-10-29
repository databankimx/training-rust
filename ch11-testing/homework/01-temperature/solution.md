## Solution: Temperature

This walkthrough explains the example solution in `src/main.rs`. Read it after you've written your own version.

### Run the solution

Press **Run** for the program's output, and **Test** for the tests. The output is:

```
0.0 C -> Ok(32.0)
100.0 C -> Ok(212.0)
-40.0 C -> Ok(-40.0)
-300.0 C -> Err("-300 C is below absolute zero")
```

### How it works

The function checks the error case first. A temperature below -273.15 returns an `Err` with a message. Otherwise the formula gives the Fahrenheit value, wrapped
in `Ok`. Putting the check first means the formula never runs for an impossible temperature.

The four tests each cover one case. Three use `assert_eq!` to compare the full `Result` with the expected `Ok` value. The error test uses `assert!` with
`is_err`, which checks that the result is an error, and doesn't depend on the wording of the message.

The `-40` case is a useful check, because it's the one point where the two scales agree. A formula with a sign error would give a different answer there.

### The extensions

Converting back from Fahrenheit has its own impossible values, below -459.67 Fahrenheit, so that conversion can also be an error.

An error enum makes the tests more precise. Each test can check for its own variant, rather than matching on the text of a message.

A value just above absolute zero, such as -273.14, is valid. It checks the boundary from the other side, so a function that rejected everything below zero would be caught.

---

*This walkthrough accompanies the exercise in Chapter 11 of the course. It's not from the book.*
