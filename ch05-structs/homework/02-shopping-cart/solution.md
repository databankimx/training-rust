## Solution: Shopping Cart

This walkthrough explains the example solution in `src/main.rs`. Read it after you've tried the exercise yourself.

### Run the solution

Press **F5**. The Output pane shows:

```
apples: 150 cents
apples: 250 cents
```

### How it works

`new` takes a `&str` for the name, so the caller doesn't have to create a `String`. Inside, `String::from(name)` makes an owned copy, which
the struct keeps. The other two fields are set with field init shorthand, as in Chapter 5.1.

`name` is a getter. It takes `&self` and returns `&str`, which is a reference into the struct. The caller reads the name, and the struct keeps it.

`total_cents` reads two fields and returns the product. It takes `&self`, because it doesn't change anything.

`add_quantity` takes `&mut self`, because it changes the quantity. That means the caller needs a mutable binding. In `main`, `apples` is declared
`let mut apples`. If the `mut` is left off, the call to `add_quantity` is rejected, because the instance has no write permission.

### The extensions

`remove_quantity` is a judgment call. Subtracting more than the quantity would underflow, and with `u32` the program would panic in a debug build.
A clean option is to set the quantity to zero, using `saturating_sub`. Another is to return a `Result`, which Chapter 9 covers.

`summary` returns a `String`, because it builds new text with `format!`. Returning a `&str` would be wrong, because the text doesn't exist anywhere
the struct could borrow from.

A function that prints the total should take `&Item`. It only reads the item, and a reference lets the caller keep using the item afterwards.

---

*This walkthrough accompanies the exercise in Chapter 5 of the course. It's not from the book.*
