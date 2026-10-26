## Grouping Values and Hashing

Hash maps can also hold a collection as the value. This step groups the positions of each character, using `entry` with a vector as the value.
It also covers two points about hash maps that come up in the quiz below: the borrow rules, and the hashing function.

---

### Grouping positions

The map's values are vectors, so each character's entry is a list of the positions where it appears:

```rust,ignore
let mut positions: HashMap<char, Vec<usize>> = HashMap::new();
for (i, c) in "hello!".chars().enumerate() {
    positions.entry(c).or_insert(Vec::new()).push(i);
}
```

`or_insert(Vec::new())` returns a mutable reference to the vector for `c`, creating an empty one if `c` is new. `push(i)` then adds the position.
The `enumerate` method pairs each character with its index.

The sample looks up the positions of `'l'`, which are 2 and 3, and adds them:

```rust,ignore
let mut sum = 0;
for i in positions.get(&'l').unwrap() {
    sum += *i;
}
```

The output is:

```
5
```

`get` returns an `Option<&Vec<usize>>`. `unwrap` is safe here, because `l` is in the string. In a real program, you'd handle the `None` case
with `match` or `if let`, as Chapter 6 showed.

---

### Borrows and updates

A reference from `get` borrows the map. You can't change the map while that reference is still in use. The first quiz shows the mistake:

```rust,ignore
let mut h = HashMap::new();
h.insert("k1", 0);
let v1 = &h["k1"];
h.insert("k2", 1);
let v2 = &h["k2"];
println!("{} {}", v1, v2);
```

The program fails on line 6. `v1` is still used by the `println!`, so it borrows `h` at that point, and `h.insert` needs to change the map. The
borrow rules forbid both at once, just as they did for vectors.

---

### The default hash function

By default, a `HashMap` uses a hashing function called SipHash. It's designed to resist denial-of-service attacks, where an attacker chooses keys
that all collide and slow the map to a crawl. It's not the fastest hash available. If profiling shows it's too slow for your program, you can
choose another hasher. The book's note is that a crate from crates.io is usually a better source for a different hasher than writing one yourself.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
5
```

The comments in `src/main.rs` suggest changing `'l'` to `'z'`. The map has no entry for `z`, so `get` returns `None`, and `unwrap` panics.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Does this program compile?

```rust
use std::collections::HashMap;
fn main() {
  let mut h = HashMap::new();
  h.insert("k1", 0);
  let v1 = &h["k1"];
  h.insert("k2", 1);
  let v2 = &h["k2"];
  println!("{} {}", v1, v2);
}
```

○ This program does compile
→ Not quite. `v1` borrows `h`, and `h.insert` changes it while `v1` is still used.
◉ This program does not compile
→ Yes. The map can't be changed while the immutable reference `v1` is still used.
---
On which line does Rust report the problem?

[ 6 ]
→ Yes. Line 6 is `h.insert("k2", 1)`, which changes the map while `v1` is still in use.
````

---

*For more detail, see section 8.3 "Storing Keys with Associated Values in Hash Maps" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch08-03-hash-maps.html).*
