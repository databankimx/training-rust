## Solution: Convert a Panic

This walkthrough explains the example solution in `src/main.rs`. Read it after you've tried the exercise yourself.

### Run the solution

Press **F5**. The Output pane shows:

```
age: 42
error: "abc" is not a number: invalid digit found in string
error: age 200 is out of range
```

### How it works

`age_from` returns `Result<u8, String>`. The parse step uses `map_err` to turn the `ParseIntError` into a `String` with a message that includes the input. `?`
then returns that error straight away, so no later step runs on a value that failed to parse.

The range check is a plain early return. A value above 120 is valid for `u8`, so `parse` succeeds, and the check is the only thing that rejects it. Returning
`Err` here, rather than panicking, is what lets `main` keep going.

`main` matches on each result. A bad age prints an error, and the loop moves on to the next input. The program finishes all three inputs, which is the behavior
the exercise asked for.

### The extensions

`age_from` should return an error, because a user can type anything. A test that calls `age_from(" 42 ").unwrap()` is a different case. If the test input is
wrong, the test should fail loudly, so `unwrap` is the right tool there, as Chapter 9.3 explained.

A type like `Guess` could hold only valid ages, in which case the check runs once, in its constructor. Functions that take that type then need no checks.

Reading from the user with `read_line` adds a loop, and the blank line ends it. Bad input stays an `Err` for that one line, so the loop can carry on, which is
the same pattern as the settings exercise.

---

*This walkthrough accompanies the exercise in Chapter 9 of the course. It's not from the book.*
