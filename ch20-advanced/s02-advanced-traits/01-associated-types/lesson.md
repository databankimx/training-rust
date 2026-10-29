## Associated Types

An *associated type* is a placeholder inside a trait. The trait's methods can refer to it, and each type that implements the trait chooses the concrete type. The standard library's `Iterator` trait
is the main example. Its method `next` returns `Option<Self::Item>`, and `Item` is decided by each implementation:

```rust,ignore
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

This step implements `Iterator` for a `Counter` that yields the numbers 1 to 5:

```rust,ignore
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // ...
    }
}
```

Once the implementation is written, `Item` is `u32` every time, so the caller doesn't need to say so.

**Why not generics?** A trait with a generic parameter, such as `Iterator<Item>`, could be implemented for the same type many times, with different parameters. Then every call to `next` would need an annotation
to say which implementation was meant. An associated type can be set only once for a type, so the compiler always knows which one applies. That's why `Iterator` uses an associated type.

The sample collects the counter into a vector, and then uses several adapters on two counters:

```rust,ignore
let sum: u32 = Counter::new()
    .zip(Counter::new().skip(1))
    .map(|(a, b)| a * b)
    .filter(|x| x % 3 == 0)
    .sum();
```

The output is:

```
all counted: [1, 2, 3, 4, 5]
sum: 18
```

`zip` pairs each number with the next one, which gives the products 2, 6, 12, and 20. The filter keeps the multiples of three, 6 and 12, and their sum is 18.

---

### Try it in the runner

Press **Run**. The output should match the two lines above.

The comments in `src/main.rs` suggest raising the limit from 5 to 6. The counter then yields 1 to 6, the products are 2, 6, 12, 20, and 30, and the sum of the multiples of three is 48.

---

*For more detail, see section 20.2 "Advanced Traits" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html).*
