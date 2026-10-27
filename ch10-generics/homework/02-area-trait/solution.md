## Solution: Area Trait

This walkthrough explains the example solution in `src/main.rs`. Read it after you've tried the exercise yourself.

### Run the solution

Press **F5**. The Output pane shows:

```
area 12.57
area 20.00
```

### How it works

The trait has one required method and one default. `describe` calls `self.area()`, so it works for any type that implements `Area`, without repeating the formula.
`Rectangle` takes the default, and `Circle` could override it later, as the extension asks.

`largest` is generic over `T`, with the bound `T: Area`. The bound is what makes `item.area()` legal: without it, the compiler wouldn't know that `T` has an `area`
method. The function returns a reference to the item, so the caller keeps the collection and doesn't lose its items.

`fold` begins with `None`, the best item so far. For each item, the closure keeps the existing best if its area is at least as large, and otherwise takes the new item.
The match guard does the comparison. Because the closure's first parameter is annotated as `Option<&T>`, Rust knows what the accumulator's type is.

Each call to `largest` works on one collection with one type. The circles are one type, and the rectangles are another, so each list gets its own call.

### The extensions

Overriding `describe` for `Circle` changes what `largest(&circles).unwrap().describe()` calls. The method is chosen by the concrete type, `Circle`, because it has
its own version. The default is still used by `Rectangle`.

A `Triangle` that implements `Area` works with the existing `largest`, without changing it. The function only needs `T: Area`, and the new type meets that bound.

`largest` returns `None` only for an empty slice, because every non-empty slice has a largest item. The `Option` is there for that one case. The caller can't
get a value from an empty list, and the type makes them handle that.

---

*This walkthrough accompanies the exercise in Chapter 10 of the course. It's not from the book.*
