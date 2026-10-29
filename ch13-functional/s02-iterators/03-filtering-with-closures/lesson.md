## Closures That Capture Their Environment

Many iterator adapters take closures, and those closures often capture values from their surroundings. `filter` is a good example. It takes a closure that gets each
item and returns a `bool`. Items for which the closure returns `true` are kept, and the rest are dropped.

The sample has a list of shoes, and a function that returns only the shoes of one size:

```rust,ignore
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
```

Each part does one job:

* `into_iter` takes ownership of the vector, and gives an iterator of owned shoes. Because the shoes are owned, the function can return them.
* `filter` makes a new iterator, which keeps the shoes the closure approves of.
* The closure `|s| s.size == shoe_size` captures `shoe_size` from the function's parameters. Each shoe's size is compared with it.
* `collect` gathers the kept shoes into a `Vec`, which is returned.

The sample prints:

```
[Shoe { size: 10, style: "sneaker" }, Shoe { size: 10, style: "boot" }]
```

The sandal is size 13, so it's dropped. The two size 10 shoes are kept, in the order they appear.

The book tests this function with an assertion. Here it's printed instead, so you can see the result directly.

---

### Try it in the runner

Press **Run**. The output should match the line above.

The comments in `src/main.rs` suggest changing the requested size to 13. Think about which shoes are returned, and what the closure compares each time.

---

*For more detail, see section 13.2 "Processing a Series of Items with Iterators" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch13-02-iterators.html).*
