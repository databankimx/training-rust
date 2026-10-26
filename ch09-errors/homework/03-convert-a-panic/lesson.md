## Homework: Convert a Panic

**The problem:** The function below parses an age. It panics on bad input, which stops the whole program the first time a user mistypes an age.

```rust,ignore
fn age_from(text: &str) -> u8 {
    let age: u8 = text.trim().parse().unwrap();
    if age > 120 {
        panic!("age {age} is out of range");
    }
    age
}

fn main() {
    println!("{}", age_from(" 42 "));
}
```

Users mistype input all the time, so bad input is an expected failure, not a bug. Change `age_from` to return `Result<u8, String>`, so that bad input becomes an
`Err` with a message. Then change `main` to handle the result, so it can keep going.

Try converting it yourself before you look at the example.

---

### Hints

* Change the return type first, then let the compiler tell you what needs changing. Every return value needs `Ok(...)` or `Err(...)`.
* `parse` returns a `Result`, whose error type is `ParseIntError`. `map_err` turns that error into a `String`, so it fits the function's return type.
* `?` can then pass the error back up, just as in the scores exercise.
* A value above 120 needs an early `return Err(...)`, where the panic was.
* In `main`, handle each input with `match`, so that a bad input prints an error and the loop continues.

---

### Test your code

Your program should check three inputs, `" 42 "`, `"abc"`, and `"200"`, and print:

```
age: 42
error: "abc" is not a number: invalid digit found in string
error: age 200 is out of range
```

The program should finish all three inputs, rather than stopping at the first bad one.

---

### Extensions

* Explain why `age_from` should return an error rather than panic, but a test that calls `age_from(" 42 ").unwrap()` may panic. Which is the right tool in each place?
* Move the range check into a `Guess`-style type, like the one in Chapter 9.3, so that only valid ages can be created.
* Make `main` read the ages from the user with `read_line`, and loop until the user enters a blank line.

---

*This exercise is not from the book. It practices converting a panic into a `Result`, from Chapter 9.*
