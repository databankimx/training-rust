## Ownership and Updating Hash Maps

A hash map owns the keys and values you insert. How you insert and update them decides what happens to existing entries, and it decides whether
the values you gave are still usable afterwards.

---

### Ownership

For types that are `Copy`, such as `i32`, the value is copied into the map. For owned types such as `String`, the map takes ownership. After
`insert(field_name, field_value)`, neither variable can be used, because they've been moved into the map.

If you insert references instead, the values aren't moved. But the values the references point to must live at least as long as the map. The
book covers that rule with lifetimes in Chapter 10.

---

### Overwriting

Each key holds one value at a time. Inserting a key that already exists replaces the old value. Inserting `("Blue", 10)` and then `("Blue", 25)`
leaves a single entry with the value 25, and the 10 is gone.

---

### entry and or_insert

Often you want to insert a value only when the key is missing. The `entry` method takes the key, and returns an `Entry` that says whether the key
exists. `or_insert` adds a default value if the key is missing, and returns a mutable reference to the value either way:

```rust,ignore
let count = counts.entry(word).or_insert(0);
*count += 1;
```

If the word is new, `or_insert(0)` adds it with zero, and the `*count += 1` makes it one. If the word is already there, the existing count is
returned, and it goes up by one. The `*` dereferences the mutable reference to change the number.

The sample counts the words in a sentence this way. The output is:

```
world: 2
hello: 1
wonderful: 1
```

The sample prints single keys rather than the whole map. A hash map's entries come out in an arbitrary order, and that order can change between runs.
Looking up one key at a time gives output you can rely on.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
world: 2
hello: 1
wonderful: 1
```

The comments in `src/main.rs` suggest inserting `"hello"` with the value 25. The old count is overwritten, so `hello` prints 25, as the overwriting
section described.

---

*For more detail, see section 8.3 "Storing Keys with Associated Values in Hash Maps" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch08-03-hash-maps.html).*
