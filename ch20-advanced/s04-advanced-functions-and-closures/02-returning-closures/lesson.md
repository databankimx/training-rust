## Returning Closures

A closure has no name you can write in a type, because each closure is its own type. Functions that return a closure therefore use a *trait* in the return type. The first option is `impl Fn`:

```rust,ignore
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}
```

The caller doesn't need to know the closure's type, only that it implements `Fn(i32) -> i32`. The closure captures `n`, so `move` moves it into the closure. `make_adder(10)` returns a closure that adds 10:

```rust,ignore
let add_ten = make_adder(10);
println!("make_adder: {}", add_ten(5));
```

The output is `make_adder: 15`.

**Several closures in one collection.** Two functions that each return `impl Fn(i32) -> i32` produce two different types, even though the signatures match. The compiler calls these *opaque types*. A `Vec` needs one element type, so the closures
must be stored as trait objects. A `Box<dyn Fn(i32) -> i32>` gives every closure the same type:

```rust,ignore
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}

let handlers = vec![returns_closure(), returns_initialized_closure(123)];
```

Each closure is called through the trait object, so `handler(5)` gives 6 for the first, and 128 for the second. The output is:

```
make_adder: 15
handler: 6
handler: 128
```

This costs one heap allocation per closure, and a dynamic call each time. Chapter 18 covered trait objects in general.

---

### Try it in the runner

Press **Run**. The output should match the three lines above.

The comments in `src/main.rs` suggest changing the return type of `returns_closure` to `impl Fn(i32) -> i32`, and putting it in the same vector. The compiler reports mismatched types, because the two closures have different opaque types, and only the `Box` version gives them one type.

---

*For more detail, see section 20.4 "Advanced Functions and Closures" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch20-04-advanced-functions-and-closures.html).*
