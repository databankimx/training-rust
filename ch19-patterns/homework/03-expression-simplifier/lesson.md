## Homework: An Expression Simplifier

**The problem:** Simplify arithmetic expressions. Adding zero, multiplying by one, multiplying by zero, and adding or multiplying two numbers should all be simplified. Write the simplifier with nested patterns, so the rules read
directly from the code.

Create a project in VS Code with `cargo new expression_simplifier`. In `src/main.rs`:

* Define `enum Expr` with four variants: `Num(i64)`, `Var(String)`, `Add(Box<Expr>, Box<Expr>)`, and `Mul(Box<Expr>, Box<Expr>)`.
* Write `fn simplify(expr: Expr) -> Expr`. Simplify the two operands first, and then match on the pair of results.
  * Two numbers add or multiply to a single number.
  * For `Add`, a zero on either side leaves the other side. Use an or-pattern, `(Num(0), e) | (e, Num(0))`, so both orders are covered.
  * For `Mul`, a zero on either side gives zero, and a one on either side leaves the other side.
  * Anything else is rebuilt with `Box::new`.
* Write `fn show(expr: &Expr) -> String` that prints an expression, with parentheses around each operation, such as `(2 + 3)`.
* In `main`, simplify `(0 + (y * 1))` and `((2 + 3) * x)`, and print each one before and after.

Write tests in a `tests` module. Check that adding zero removes it, that multiplying by zero gives zero, and that two numbers are added.

---

### Hints

* Matching on `(simplify(*a), simplify(*b))` moves both results into a tuple, so the patterns can bind them by value.
* In an or-pattern, each alternative must bind the same names with the same types, so `e` appears on both sides of `|`.
* Put the `(Num(x), Num(y))` arm first. Then the zero and one arms only see expressions that aren't both numbers.

---

### Check your work

Your program should print:

```
before: (0 + (y * 1))
after: y
before: ((2 + 3) * x)
after: (5 * x)
```

---

### Extensions

* Add `Sub(Box<Expr>, Box<Expr>)`. Which rule should it have for `x - 0`, and what happens to `0 - x`?
* Apply `simplify` repeatedly until the expression no longer changes. Is one pass always enough for this set of rules?

---

*This exercise is not from the book. It practices nested patterns and or-patterns from Chapter 19.*
