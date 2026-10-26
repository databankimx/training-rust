## Solution: Traffic Light

This walkthrough explains the example solution in `src/main.rs`. Read it after you've tried the exercise yourself.

### Run the solution

Press **F5**. The Output pane shows:

```
Red for 30 seconds
Green for 25 seconds
Yellow for 5 seconds
Red for 30 seconds
```

### How it works

`Light` derives `Copy` and `Clone`. Because of that, `next(self)` takes the light by value without moving it. The caller's `light` is copied, and
the assignment `light = light.next()` replaces the variable with the next light.

`next` has one arm for each variant, and it returns the following light. Since `match` is exhaustive, adding a variant without an arm is a compile error.

`seconds` takes `&self`. It only reads the variant, so the caller doesn't need to give the light up, and the `match` returns a number for each variant.

The loop prints the current light and its duration, and then moves on. The `_` in `for _ in 0..4` means the loop counter isn't needed.

### The extensions

`is_go` can use `matches!(self, Light::Green)`, which is a short form of a `match` that returns `true` or `false`. A `match` with two arms is also clear,
and some people prefer it because it's easier to extend.

A `Flashing` variant makes every `match` on `Light` incomplete. The compiler names each one, and you add an arm to each. This is the exhaustiveness check
working as intended.

Skipping yellow at night needs information the method doesn't have: the time of day, or whether it's night. A method can take that as a parameter, such as
`next(self, is_night: bool)`, rather than storing it in the enum.

---

*This walkthrough accompanies the exercise in Chapter 6 of the course. It's not from the book.*
