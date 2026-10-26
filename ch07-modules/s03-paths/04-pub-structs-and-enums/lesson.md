## Making Structs and Enums Public

`pub` works on structs and enums, but a struct and an enum follow different rules for their parts.

---

### Structs: each field is separate

Marking a struct `pub` makes the struct public, but its fields stay private. You make each field public or private on its own. The sample models a
restaurant breakfast: the customer picks the toast, so `toast` is `pub`, and the chef picks the fruit, so `seasonal_fruit` is private:

```rust,ignore
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}
```

Because `seasonal_fruit` is private, `eat_at_restaurant` can't set it. So the struct provides a public constructor, `summer`, which sets the private
field inside the module:

```rust,ignore
pub fn summer(toast: &str) -> Breakfast {
    Breakfast {
        toast: String::from(toast),
        seasonal_fruit: String::from("peaches"),
    }
}
```

Without a constructor like this, nothing outside the module could make a `Breakfast`. A public method, `fruit`, reads the private field, so callers can
still see which fruit they got.

The sample's output is:

```
I'd like Wheat toast please
Today's fruit is peaches
```

---

### Enums: all variants are public

Marking an enum `pub` makes every variant public as well. You don't need to annotate each variant. This makes sense, because an enum isn't much use if
its variants can't be named. Structs are different, because a struct is often useful with some of its fields hidden.

```rust,ignore
pub enum Appetizer {
    Soup,
    Salad,
}
```

Code outside the module can now write `Appetizer::Soup` and `Appetizer::Salad`.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
I'd like Wheat toast please
Today's fruit is peaches
```

The comments in `src/main.rs` suggest setting `meal.seasonal_fruit` directly from `eat_at_restaurant`. The compiler rejects that, because the field is private
to the `back_of_house` module.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Does this program compile?

```rust
pub mod point {
  #[derive(Debug)]
  pub struct Point(i32, i32);

  impl Point {
    pub fn origin() -> Self { Point(0, 0) }
  }
}

fn main() {
  let mut p = point::Point::origin();
  p.0 += 1;
  println!("{p:?}");
}
```

○ This program does compile
→ Not quite. The fields of `Point` are private, so `p.0` can't be used from `main`.
◉ This program does not compile
→ Yes. The struct is public, but its fields are not, so `main` can't change `p.0`.
---
On which line does Rust report the problem?

[ 12 ]
→ Yes. Line 12 is `p.0 += 1`, which uses a private field from outside the module.
````

---

*For more detail, see section 7.3 "Paths for Referring to an Item in the Module Tree" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html).*
