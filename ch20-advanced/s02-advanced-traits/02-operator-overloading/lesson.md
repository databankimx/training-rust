## Operator Overloading and Default Type Parameters

Rust lets you customize what an operator such as `+` does, by implementing the trait behind it in `std::ops`. Rust doesn't let you create new operators, or overload every operator, but any operator
with a trait in `std::ops` can be given a meaning for your own types.

The `Add` trait is declared with a default type parameter:

```rust,ignore
trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}
```

`Rhs` stands for "right-hand side", the type of the value on the right of the `+`. The `= Self` part is a *default*. If an implementation doesn't name `Rhs`, the right-hand side is the same type as the
left. That is what makes `Point + Point` work without any extra syntax:

```rust,ignore
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
```

Sometimes the right-hand side should be a different type. Here, adding `Meters` to `Millimeters` needs a conversion, so the implementation sets `Rhs` to `Meters` explicitly:

```rust,ignore
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + other.0 * 1000)
    }
}
```

The output is:

```
Point { x: 3, y: 3 }
2500mm
```

The `Millimeters` and `Meters` types are *newtypes*, which are thin wrappers around a number. The step after next covers the newtype pattern in more detail.

**Why defaults?** A default type parameter lets a trait be extended without breaking existing code, and it lets most users write the simple form. `Add` is the usual example: most additions are between two of the same type,
so the default is the common case, and an explicit `Rhs` is available when a different type is needed.

---

### Try it in the runner

Press **Run**. The output should match the two lines above.

The comments in `src/main.rs` suggest changing `Meters(2)` to `Meters(0)`. Zero meters adds nothing, so the total is `500mm`.

---

*For more detail, see section 20.2 "Advanced Traits" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html).*
