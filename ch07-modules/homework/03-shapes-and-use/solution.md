## Solution: Shapes and use

This walkthrough explains the example solution in `src/main.rs`. Read it after you've tried the exercise yourself.

### Run the solution

Press **F5**. The Output pane shows:

```
circle area: 12.56
square area: 9
small square area: 1
```

### How it works

`pub use shapes::round::Circle;` brings `Circle` to the crate root, so `main` can write `Circle` directly. The `pub` lets code that can reach the
root use the same name. In a binary crate, that's only the crate itself, but in a library it would be the public API.

`use shapes::flat::{self, Square};` brings in two names. `Square` is the struct, and `self` brings in `flat` itself. Because `flat` is in
scope, `main` can write `flat::Square` as well. Both forms refer to the same struct, so the program uses them side by side.

The arithmetic is straightforward. A circle of radius 2 has area `3.14 * 2 * 2`, which is 12.56. A square of side 3 has area 9. The small square of side 1
has area 1. Floating-point values print in their shortest exact form, so the output has no trailing zeros.

### The extensions

A plain `use` for `Circle` still compiles in this binary. But if `Circle` were used from another crate, the name would be private, and the caller
couldn't reach it through the crate root. `pub use` is what makes the re-export useful in a library.

A `shapes::solid` module with a `Cube` struct would follow the same pattern. The nested `use` could be written as `use shapes::{flat, solid::{self, Cube}};`,
which brings in `flat`, `solid`, and `Cube` in one line.

Removing `flat` from the `use` line and writing `Square` in `main` still works, because `Square` is in scope. Removing `Square` and writing `flat::Square`
without the `flat` name in scope would fail, because `flat` would no longer be a name in `main`.

---

*This walkthrough accompanies the exercise in Chapter 7 of the course. It's not from the book.*
