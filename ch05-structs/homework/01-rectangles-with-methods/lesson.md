## Homework: Rectangles with Methods

**The problem:** Start with a `Rectangle` struct that has a `width` and a `height`. Add three things to it:

* An associated function `square(size: u32) -> Self`, which makes a square rectangle.
* A method `perimeter(&self) -> u32`, which returns the total length of the four sides.
* A method `is_square(&self) -> bool`, which returns `true` when the width and height are equal.

Then write a `main` that prints the perimeter and whether each rectangle is a square, for a rectangle of 30 by 50 and a square of side 4.

Try writing it yourself before you look at the example.

---

### Hints

* Put all three in an `impl Rectangle` block. The associated function has no `self`, so it's called with `::`.
* `square` can build a `Self` and set both fields to `size`.
* The perimeter is the sum of two sides, doubled. Think about whether `perimeter` needs to change the rectangle. If it doesn't, what should the first parameter be?
* `is_square` can compare the two fields with `==`.

---

### Test your code

Your program should print these four lines:

```
rect perimeter: 160
rect is square: false
sq perimeter: 16
sq is square: true
```

Check the numbers by hand. A 30 by 50 rectangle has a perimeter of 2 times (30 + 50).

---

### Extensions

* Add a method `double(&self) -> Rectangle` that returns a new rectangle with both sides doubled. Why does it return a new rectangle, and not change the original?
* Add a method `set_width(&mut self, width: u32)`. Which variable in `main` has to be declared `mut`?
* Add a constructor `new(width: u32, height: u32) -> Self`, and use it in place of the struct literal in `main`.

---

*This exercise is not from the book. It practices associated functions and methods from Chapter 5.*
