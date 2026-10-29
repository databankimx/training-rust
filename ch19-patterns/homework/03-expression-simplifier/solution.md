## Solution: An Expression Simplifier

This walkthrough explains the example solution in `src/main.rs`. Read it after you've written your own version.

### Run the solution

Press **Run** for the output, and **Test** for the three tests. The output is:

```
before: (0 + (y * 1))
after: y
before: ((2 + 3) * x)
after: (5 * x)
```

### How it works

`simplify` takes the expression by value. For an operation, it simplifies both operands first, and then matches the pair of results. Matching a tuple of owned values lets each pattern move
the parts it binds, so the simplified operands aren't copied.

For `Add`, the first arm handles two numbers. The or-pattern `(Num(0), e) | (e, Num(0))` handles a zero on either side. Both alternatives bind `e`, with the same type, which is required for an
or-pattern. The last arm rebuilds the sum when no rule applies.

For `Mul`, the rules come in order. Two numbers multiply. A zero on either side gives zero, and that arm uses `_` for the other side, because it doesn't matter. A one on either side leaves the
other side. The order matters: `Num(0)` is checked before `Num(1)`, so a product with a zero is always zero.

Each arm's pattern only sees expressions that weren't caught by an earlier arm. The `(Num(x), Num(y))` arm comes first, so the zero and one arms never see two numbers.

The output shows the rules at work. The first expression loses its zero and its one. The second adds 2 and 3 to 5, and keeps the variable, because no rule applies to `5 * x`.

### The extensions

For `Sub`, `x - 0` simplifies to `x`, which is the same as the addition rule on the right-hand side. `0 - x` is not the same as `x`, because subtraction isn't commutative, so only the right-hand zero is a rule.

Because the operands are simplified before the outer rule runs, `(x * 0) + y` becomes `y` in a single pass. The inner product is already `0` when the addition rule sees it. A single pass isn't always enough when a rule
creates a new shape that an earlier rule could have changed. Repeating `simplify` until the expression stops changing is the safe way to handle that case, and it's worth adding if you extend the rules.

---

*This walkthrough accompanies the exercise in Chapter 19 of the course. It's not from the book.*
