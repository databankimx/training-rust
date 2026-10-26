## Homework: Traffic Light

**The problem:** Model a traffic light as an enum with three variants: `Red`, `Green`, and `Yellow`. Give it two methods:

* `next(self) -> Light`, which returns the light that follows this one: red to green, green to yellow, and yellow back to red.
* `seconds(&self) -> u32`, which returns how long the light stays on: 30 for red, 25 for green, and 5 for yellow.

Then write a `main` that starts at red, and prints the light and its duration four times, moving to the next light after each one.

Try writing it yourself before you look at the example.

---

### Hints

* Each method needs a `match` with one arm per variant. The compiler will tell you if you miss one.
* `next` can take `self` by value, if the enum derives `Copy`. Otherwise the caller would lose the light after calling it.
* `seconds` only reads the light, so it should take `&self`.
* To print the variant's name, derive `Debug` on the enum.

---

### Test your code

Your program should print these four lines:

```
Red for 30 seconds
Green for 25 seconds
Yellow for 5 seconds
Red for 30 seconds
```

The sequence wraps around, so after yellow comes red again.

---

### Extensions

* Add a method `is_go(&self) -> bool` that returns `true` only for green. Write it with `matches!`, or with a `match`. Which do you find clearer?
* Add a fourth variant, `Flashing`, that lasts 2 seconds. What does the compiler say about the `match` arms, and which one must you update?
* Make `next` skip `Yellow` when the light is `Green` during the night. What information does the method need that it doesn't have now?

---

*This exercise is not from the book. It practices enums and `match` from Chapter 6.*
