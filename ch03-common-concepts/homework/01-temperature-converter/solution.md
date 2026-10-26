## Solution: Temperature Converter

This walkthrough explains the example solution in `src/main.rs`. Read it after you've tried the
exercise yourself. Your version may be quite different, and that's fine. Several ways of solving the
problem are correct.

### Run the solution

Press **F5**, and type a temperature, such as `100`, then `C`. The Output pane shows:

```
Temperature converter
Enter the temperature:
100
Is it in F or C?
C
100.0°C = 212.0°F
```

Try `212` and `F`. You should get `100.0°C` back.

### How it works

The program is split into small functions, each with one job:

* `read_temperature` asks until the player types a number. It uses a `loop` and returns the value
  from inside it with `return`.
* `read_scale` asks until the player types `F` or `C`. It uses `to_uppercase`, so `f` and `F` both work.
* `fahrenheit_to_celsius` and `celsius_to_fahrenheit` do the arithmetic. Each takes one `f64` and returns
  one `f64`, which is the function signature you learned in the functions step.
* `read_line` does the input, the same way we did in the guessing game.

The `if` in `main` chooses which conversion to show. The two conversions are written as separate
functions, so each is easy to test and easy to read.

The `{value:.1}` in the `println!` line means "show one decimal place". Without it, Rust would print
as many decimal places as the number needs.

---

*This walkthrough accompanies the exercise in [The Rust Programming Language](https://rust-book.cs.brown.edu/ch03-05-control-flow.html#summary).*
