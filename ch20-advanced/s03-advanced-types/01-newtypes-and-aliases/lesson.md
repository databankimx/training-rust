## Newtypes, Type Aliases, and Type Safety

Chapter 20.2 introduced the newtype pattern as a way around the orphan rule. Newtypes have another use: they stop values from being confused. A newtype is a different type, so the compiler refuses to
use one where another is expected, even when both wrap the same number.

```rust,ignore
struct Millimeters(u32);
struct Meters(u32);

fn to_millimeters(m: Meters) -> Millimeters {
    Millimeters(m.0 * 1000)
}
```

`to_millimeters` accepts only `Meters`. Passing a plain `u32`, or a `Millimeters`, would not compile. The units are part of the type, so mistakes like this are caught before the program runs.

**Type aliases** work differently. A `type` declaration gives an existing type another name, and does not create a new type:

```rust,ignore
type Kilometers = i32;
```

`Kilometers` is `i32`, so the two can be mixed freely. The compiler gives no protection in this case. Use an alias to make code clearer, and a newtype when the compiler should keep the values apart.

**Reducing repetition.** Aliases are most useful for long types that appear in many places. The book's example is a boxed closure:

```rust,ignore
type Thunk = Box<dyn Fn() + Send + 'static>;

fn run_thunk(f: Thunk) {
    f();
}
```

The alias makes the signatures shorter, and the name `Thunk` says what the value is for. The standard library uses the same idea for `io::Result<T>`, which is `Result<T, io::Error>` with the error filled in.

The output is:

```
kilometers: 15
thunk runs
millimeters: 2000
```

---

### Try it in the runner

Press **Run**. The output should match the three lines above.

The comments in `src/main.rs` suggest calling `to_millimeters` with a plain number. The compiler rejects it, because the function needs a `Meters` value, not an integer.

---

*For more detail, see section 20.3 "Advanced Types" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch20-03-advanced-types.html).*
