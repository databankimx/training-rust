## Homework: Money Operators

**The problem:** Money should be stored in cents, so that adding and multiplying amounts never loses a fraction of a cent. Write a `Cents` newtype that supports `+`, `-`, and `*` by a whole number, and that prints
as dollars and cents, such as `$38.49`.

Create a project in VS Code with `cargo new money`. In `src/main.rs`:

* Define `struct Cents(i64);`, and derive `Debug`, `Clone`, `Copy`, and `PartialEq`.
* Implement `Add<Cents>` and `Sub<Cents>` for `Cents`. Leaving out `Rhs` makes the right-hand side `Cents`, as in the default type parameter from Chapter 20.2.
* Implement `Mul<i64>` for `Cents`, so that `price * 3` works. Here `Rhs` must be set explicitly, because it's not `Cents`.
* Implement `Display` so that `Cents(3849)` prints `$38.49`. Use `/` and `%` by 100, and `{:02}` to pad the cents to two digits.
* In `main`, compute the total of `Cents(1250) * 3 + Cents(99)`, and the change from `Cents(5000)`, and print both.

Write tests in a `tests` module. Check addition, subtraction, multiplication, and the display of `Cents(5)`, which should print `$0.05`.

---

### Hints

* `impl Add for Cents` and `impl Add<Cents> for Cents` are the same. The default makes the first one shorter.
* The `Output` associated type of each operator trait is `Cents` for all of these.
* Inside `fmt`, write with `write!(f, "${}.{:02}", dollars, cents)`.

---

### Check your work

Your program should print:

```
total: $38.49
change: $11.51
```

Three items at 1250 cents come to 3750, plus 99 is 3849, which is $38.49. Then 5000 less 3849 is 1151, which is $11.51.

---

### Extensions

* Add `Neg`, so that `-Cents(100)` works. What should `Display` print for a negative amount?
* Could you implement `Mul<Cents>` for `i64`, so that `3 * Cents(1250)` works? What does the orphan rule say about that?

---

*This exercise is not from the book. It practices operator overloading and newtypes from Chapter 20.*
