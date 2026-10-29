## Homework: Fix the Ownership Error

**The problem:** The program below is rejected by the compiler. It tries to call a method that takes `self` through a mutable reference.

```rust,ignore
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn wider(self, by: u32) -> Rectangle {
        Rectangle {
            width: self.width + by,
            height: self.height,
        }
    }
}

fn grow(rect: &mut Rectangle) {
    *rect = rect.wider(10);
}

fn main() {
    let mut r = Rectangle {
        width: 30,
        height: 50,
    };

    grow(&mut r);
    println!("{:?}", r);
}
```

The compiler reports:

```text
error[E0507]: cannot move out of `*rect` which is behind a mutable reference
```

The program should print `Rectangle { width: 40, height: 50 }`.

---

### Step 1: Try the fix that Chapter 5 mentions

The step on methods and ownership showed that `#[derive(Copy, Clone)]` turns a move into a copy. Add it to `Rectangle`, and run the program again.

Then answer: why does the derive fix the error? What does `Copy` allow the compiler to do with `*rect`?

---

### Step 2: The fix that always works

Now give `Rectangle` a `name` field, of type `String`:

```rust,ignore
#[derive(Debug, Copy, Clone)]
struct Rectangle {
    name: String,
    width: u32,
    height: u32,
}
```

The compiler now rejects the `derive`, with this error:

```text
error[E0204]: the trait `Copy` cannot be implemented for this type
```

Explain, in your own words, why a `String` field makes `Copy` impossible. Then fix the program so it compiles without `Copy`. The fix is to change `wider` to take `&self` instead of `self`, and to copy the name with `clone`.

---

### Hints

* `Copy` can only be derived when every field is `Copy`. Which field is not?
* A method that takes `&self` only borrows the rectangle, so it can't move anything out of `*rect`.
* A `String` can't be copied bit by bit, because it owns heap data. `clone` makes a new `String` with its own heap data.

---

### Test your fix

Your final program should print:

```
Rectangle { name: "window", width: 40, height: 50 }
```

---

### Extensions

* Explain why `grow` can't simply return the new rectangle, and why `*rect = ...` is needed.
* Would `#[derive(Copy, Clone)]` be allowed on a struct whose fields are only numbers and `bool` values? Why?
* Change `wider` to take `&mut self`, and update the width in place. Does `grow` still need to assign to `*rect`?

---

*This exercise is not from the book. It practices the ownership rules for methods from Chapter 5.*
