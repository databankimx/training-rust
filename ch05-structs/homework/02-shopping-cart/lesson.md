## Homework: Shopping Cart

**The problem:** Write a struct for an item in a shopping cart. It has a name, a price in cents, and a quantity. Give it:

* A constructor, `new(name: &str, price_cents: u32, quantity: u32) -> Self`.
* A getter, `name(&self) -> &str`, which returns the name.
* A method, `total_cents(&self) -> u32`, which returns the price times the quantity.
* A method, `add_quantity(&mut self, extra: u32)`, which increases the quantity.

Then write a `main` that creates three apples at 50 cents each, prints the name and total, adds two more apples, and prints again.

Prices are in cents, which are whole numbers, so the output has no rounding.

Try writing it yourself before you look at the example.

---

### Hints

* The name is a `String` inside the struct. The constructor takes a `&str`, so it has to make its own `String`.
* A getter returns a reference, so its return type is `&str`. It only needs to read the name.
* Think about which methods change the item, and which only read it. The ones that change it need `&mut self`, and the caller needs a `mut` binding.
* In `main`, decide which variables need `mut`, and check that the compiler agrees.

---

### Test your code

Your program should print these two lines:

```
apples: 150 cents
apples: 250 cents
```

The first total is 3 times 50. After adding two more, the quantity is 5, so the total is 5 times 50.

---

### Extensions

* Add a method `remove_quantity(&mut self, fewer: u32)` that decreases the quantity. What should happen if `fewer` is larger than the quantity? Decide, and make your code do it.
* Add a method `summary(&self) -> String` that returns a line such as `apples x5 = 250 cents`. What does the method need to return, and why?
* Write a function that takes a `&Item`, and prints its total. Why is a reference the right parameter type?

---

*This exercise is not from the book. It practices structs, getters, and `&mut self` from Chapter 5.*
