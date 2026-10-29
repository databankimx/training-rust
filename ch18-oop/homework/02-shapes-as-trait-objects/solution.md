## Solution: Shapes as Trait Objects

This walkthrough explains the example solution in `src/main.rs`. Read it after you've written your own version.

### Run the solution

Press **Run** for the output, and **Test** for the two tests. The output is:

```
circle: 3.14
rectangle: 6.00
circle: 0.79
total: 9.93
```

### How it works

`Shape` is the trait, and each struct implements it with its own `name` and `area`. The vector holds `Box<dyn Shape>`, so one collection can contain circles and rectangles together. The
annotation on the vector is needed for the reason section 18.2 gave: the compiler would otherwise fix the type from the first element.

`total_area` takes a slice of trait objects. Calling `area` on each element is dynamic dispatch: the program looks up the right method for each shape at runtime. The `sum` adds the results, and the type
comes from the return type of the function.

The output uses `{:.2}` to round each area to two decimal places. The total is the sum of the unrounded areas, rounded once at the end, so the printed numbers may not add up exactly to the printed total.

The tests check one area directly, and the total of an empty list. The empty list is written `&[]`, and the compiler infers its element type from the function's parameter.

### The extensions

`f64` isn't `Ord`, because `NaN` has no order with other numbers. `sort` needs `Ord`, so it can't compare `f64` values. `sort_by` takes a closure, and `partial_cmp` returns an `Option<Ordering>`, which is `None` for `NaN`. Using `partial_cmp(...).unwrap()` in the closure sorts by area, and panics if a `NaN` appears.

Adding a `Triangle` doesn't change `total_area`. It only needs another struct that implements `Shape`, and the function already works with any shape in the list. This is the open design that trait objects allow.

---

*This walkthrough accompanies the exercise in Chapter 18 of the course. It's not from the book.*
