## Homework: Area Trait

**The problem:** Define a trait, `Area`, with one required method and one default method:

* `area(&self) -> f64` is required. Each type that implements `Area` must provide it.
* `describe(&self) -> String` has a default body that returns the text `area` followed by the area to two decimal places, such as `area 12.57`.

Implement `Area` for two types, `Circle` (with a `radius`) and `Rectangle` (with a `width` and a `height`). Circles use 3.14159 as pi. Then write a generic
function, `largest<T: Area>(items: &[T]) -> Option<&T>`, that returns the item with the largest area, or `None` for an empty slice.

Finally, in `main`, print the description of the largest circle among two circles, and the largest rectangle among two rectangles.

Try writing it yourself before you look at the example.

---

### Hints

* The trait's default method can call `self.area()`, even though `area` has no default body.
* `largest` can use `fold`, which starts with `None` and keeps the better of the best item so far and the next item.
* Inside the closure, the `match` compares the two areas. An item with a larger or equal area is kept, and the other is dropped.
* `format!("{:.2}", x)` prints a number with two decimal places.

---

### Test your code

Your program should print these two lines:

```
area 12.57
area 20.00
```

The largest circle has radius 2, with area 3.14159 times 4, which is 12.57 to two decimal places. The largest rectangle is 2 by 10, with area 20.

---

### Extensions

* Override `describe` for `Circle`, so it prints `circle with area 12.57`. Which method does `largest(&circles).unwrap().describe()` call now?
* Add a `Triangle` type. Does `largest` need to change to accept it, and why not?
* Make `largest` return `None` only for an empty slice, and explain why it doesn't need `Option` for anything else.

---

*This exercise is not from the book. It practices traits, default methods, and trait bounds from Chapter 10.*
