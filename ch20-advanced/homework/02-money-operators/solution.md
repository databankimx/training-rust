## Solution: Money Operators

This walkthrough explains the example solution in `src/main.rs`. Read it after you've written your own version.

### Run the solution

Press **Run** for the output, and **Test** for the four tests. The output is:

```
total: $38.49
change: $11.51
```

### How it works

`Cents` wraps an `i64` number of cents. Each operator is an implementation of its trait, with `Output` set to `Cents`.

`Add` and `Sub` leave out `Rhs`, so the right-hand side is `Cents`, the default from Chapter 20.2. `Mul` sets `Rhs` to `i64`, because a count isn't money. So `Cents(1250) * 3` multiplies by a count, while
`Cents(1250) * Cents(3)` doesn't compile.

The `Display` implementation divides by 100 to get the dollars, and takes the remainder for the cents. The `{:02}` pads the cents to two digits, which is why `Cents(5)` prints `$0.05` and not `$0.5`.

In `main`, the total is computed with the operators, and the change is the difference from `Cents(5000)`. Each operator returns a new `Cents`, so no values are changed in place.

The tests check each operator on its own, and the display of a small amount, where the padding matters.

### The extensions

Implementing `Neg` gives `-Cents(100)`. A negative amount needs a sign, and dividing a negative number by 100 with `/` and `%` gives values such as `-1` and `-0` for a cents part, so `Display` needs
to take the absolute value and print the sign separately.

`impl Mul<Cents> for i64` is allowed. The orphan rule forbids implementing a foreign trait for a foreign type, but here `Cents` is a local type in the trait's parameter, so the implementation is in this crate, and
the rule is met. It lets `3 * Cents(1250)` work, as an alternative to `Cents(1250) * 3`.

---

*This walkthrough accompanies the exercise in Chapter 20 of the course. It's not from the book.*
