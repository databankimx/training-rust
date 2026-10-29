## Solution: Rectangles with Methods

This walkthrough explains the example solution in `src/main.rs`. Read it after you've tried the exercise yourself.

### Run the solution

Press **F5**. The Output pane shows:

```
rect perimeter: 160
rect is square: false
sq perimeter: 16
sq is square: true
```

### How it works

`square` is an associated function. It has no `self`, so it's called on the type with `::`, as in `Rectangle::square(4)`. It builds a
`Self`, which means `Rectangle`, with both fields set to the same size.

`perimeter` and `is_square` take `&self`. They only read the fields, so the caller keeps ownership and can call them again. This is the usual
choice for methods that don't change the instance. If `perimeter` took `self` instead, the first call would move `rect`, and the `println!`
for `is_square` on the next line would be rejected.

`is_square` compares the two fields with `==`, and returns the `bool` directly.

### The extensions

`double` should return a new `Rectangle`, rather than change the original, because it takes `&self`. A method that takes `&self` can't
change the instance. To change it, the method would have to take `&mut self`, and the caller would need a `mut` binding.

`set_width` takes `&mut self`, so the variable it's called on must be declared `mut`, as with any mutable borrow.

---

*This walkthrough accompanies the exercise in Chapter 5 of the course. It's not from the book.*
