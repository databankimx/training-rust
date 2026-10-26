## Solution: Fix the Ownership Error

This walkthrough explains the example solution in `src/main.rs`. Read it after you've tried the exercise yourself.

### Run the solution

Press **F5**. The Output pane shows:

```
Rectangle { name: "window", width: 40, height: 50 }
```

### Why the original is rejected

`wider` takes `self` by value. Calling `rect.wider(10)` moves the rectangle out of `*rect`, which the `grow` function only borrows. Rust doesn't allow
that, because the caller still owns the rectangle, and taking it away would leave `*rect` empty. The error message is the same one you met in
Chapter 4.3, when moving out of a reference.

### Step 1: derive(Copy, Clone)

With only numeric fields, the struct can implement `Copy`. A copy of `*rect` is made, and the original stays in place, so the move becomes a copy and
the program compiles. `Copy` is only available when every field is `Copy`, so this fix depends on the struct's contents.

### Step 2: why String blocks Copy

A `String` owns heap data. Copying it bit by bit would make two owners of the same heap buffer, and both would free it, which is a double-free.
So `String` can't be `Copy`, and any struct containing one can't be `Copy` either. The compiler reports E0204 for that reason.

The fix that always works is to borrow. `wider` takes `&self`, so it reads the rectangle without taking it. The new rectangle needs its own name, so
`self.name.clone()` makes a new `String` with its own heap data. Now both the old and new rectangles own their names, and nothing is shared.

### The extensions

`grow` can't return the new rectangle, because it has no return value, and `rect` is a reference the caller owns. `*rect = ...` writes the new value
into the place the reference points to, which is how the caller sees the change.

A struct with only numbers and `bool` values can derive `Copy`, because every one of those types is `Copy`.

If `wider` took `&mut self`, it would change `self.width` directly, and `grow` wouldn't need to assign to `*rect`. The method would then need a mutable
reference to the rectangle, and the caller would still have to pass `&mut r`.

---

*This walkthrough accompanies the exercise in Chapter 5 of the course. It's not from the book.*
