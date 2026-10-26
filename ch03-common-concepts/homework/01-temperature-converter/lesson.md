## Homework: Temperature Converter

**The problem:** Convert temperatures between Fahrenheit and Celsius.

Your program should ask for a temperature, ask whether it's in Fahrenheit or Celsius, and print the
converted value. Work on it in your own project, and try it before you look at the example.

---

### Hints

* You'll need two formulas. Work them out on paper first, then check them against a few values you know.
  Water freezes at 0°C, which is 32°F, and boils at 100°C, which is 212°F.
* Keep each job in its own function. Asking for input, doing the arithmetic, and printing the result are
  three separate jobs.
* The input from the player is text, so it needs turning into a number before you can use it. Think about
  what should happen if the player types something that isn't a number.
* Decimal numbers in Rust are `f64` by default.

---

### Test your program

Try a few values you can check by hand:

* 0 in Celsius should give 32 in Fahrenheit.
* 100 in Celsius should give 212 in Fahrenheit.
* 212 in Fahrenheit should give 100 in Celsius.

If your results don't match, check the formulas first, then check the order of the operations.

---

### Extensions

* Add a third scale, Kelvin. Kelvin is Celsius plus 273.15.
* Make the program keep converting until the player types `quit`.
* Print the converted value with no decimal places, rounding it with the `round` method on `f64`.

---

*This exercise comes from the summary at the end of Chapter 3 in [The Rust Programming Language](https://rust-book.cs.brown.edu/ch03-05-control-flow.html#summary).*
