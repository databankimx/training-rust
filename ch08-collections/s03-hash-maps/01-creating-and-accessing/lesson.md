## Creating and Accessing Hash Maps

A hash map, `HashMap<K, V>`, stores pairs of keys and values. You look up a value by its key, rather than by a position as in a vector. A
team's name can be the key, and its score the value.

---

### Creating and inserting

Hash maps aren't in the prelude, so the sample brings `HashMap` into scope first. Then it creates an empty map with `new`, and adds entries with
`insert`:

```rust,ignore
use std::collections::HashMap;

let mut scores: HashMap<String, i32> = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

Like vectors, a hash map has one key type and one value type, and stores its data on the heap. There's no `vec!`-style macro for hash maps.

---

### Reading a value

`get` takes a key and returns an `Option<&V>`. It's `None` when the key isn't in the map:

```rust,ignore
let score = scores.get(&team).copied().unwrap_or(0);
```

`copied` turns the `Option<&i32>` into an `Option<i32>`, so the number itself is returned. `unwrap_or(0)` gives zero when the key is missing. The
sample prints:

```
Blue: 10
Green: 0
```

`Green` was never inserted, so the default is used. You can also pass a string slice to `get`, as the sample does with `"Green"`, because a `String`
key can be looked up by a `&str`.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Blue: 10
Green: 0
```

The comments in `src/main.rs` suggest removing `unwrap_or(0)`. Then `score` is an `Option`, and the program prints `None` for Green.

---

*For more detail, see section 8.3 "Storing Keys with Associated Values in Hash Maps" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch08-03-hash-maps.html).*
