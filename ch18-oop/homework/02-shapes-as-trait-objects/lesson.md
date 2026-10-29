## Homework: Shapes as Trait Objects

**The problem:** A drawing program holds shapes of different kinds in one list. Each shape can report its name and its area. Write the trait, the shapes, and a function that totals the areas.

Create a project in VS Code with `cargo new shapes`. In `src/main.rs`:

* Define a trait `Shape` with two methods: `name(&self) -> String`, and `area(&self) -> f64`.
* Define a struct `Circle` with a `radius: f64`, and a struct `Rectangle` with `width: f64` and `height: f64`. Implement `Shape` for both. The area of a circle is `std::f64::consts::PI * radius * radius`.
* Write `fn total_area(shapes: &[Box<dyn Shape>]) -> f64`, which returns the sum of the areas.
* In `main`, make a `Vec<Box<dyn Shape>>` with a circle of radius 1.0, a rectangle of 3.0 by 2.0, and a circle of radius 0.5. Print each shape's name and area with two decimal places, and then the total.

Write tests in a `tests` module. Check the area of a rectangle, and that `total_area` of an empty list is zero.

---

### Hints

* The vector must be annotated, `Vec<Box<dyn Shape>>`, because the compiler infers the type from the first element otherwise, as section 18.2 showed.
* `total_area` takes a slice, so it works with a `Vec` through `&shapes`.
* `iter().map(|shape| shape.area()).sum()` adds the areas. The sum needs a type, so `let total: f64 = ...` helps.

---

### Check your work

Your program should print:

```
circle: 3.14
rectangle: 6.00
circle: 0.79
total: 9.93
```

The total is the sum of the three areas: π, 6, and π × 0.25, which together come to about 9.93.

---

### Extensions

* Sort the shapes by area before printing, using `sort_by` and `partial_cmp`. Why can't you use `sort` on `f64` values?
* Add a `Triangle` shape. Does `total_area` need to change?

---

*This exercise is not from the book. It practices trait objects from Chapter 18.*
