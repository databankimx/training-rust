## The Fn Traits

Every closure implements one, two, or all three of the traits `FnOnce`, `FnMut`, and `Fn`. Which ones depends on what its body does with the values it captured. Functions
that accept closures use these traits to say what they need.

* `FnOnce` is for closures that can be called once. Every closure implements it, because every closure can be called at least once. A closure that moves a captured
  value out of its body implements only `FnOnce`.
* `FnMut` is for closures that may change what they capture, but don't move it out. They can be called many times.
* `Fn` is for closures that neither move nor change what they capture, or capture nothing. They can be called many times, even at the same time.

The traits form a hierarchy, so a closure that implements `Fn` also implements `FnMut` and `FnOnce`. A function that needs a closure should ask for the least it
needs, so it accepts the most closures.

The standard library's `unwrap_or_else` asks for `FnOnce`, because it calls the closure at most once. `sort_by_key` asks for `FnMut`, because it calls its closure once
for every item.

The sample sorts an array of rectangles by width:

```rust,ignore
list.sort_by_key(|r| r.width);
```

The closure only reads `r.width`, so it's an `Fn` closure, and it works wherever `FnMut` is needed. The program prints the rectangles in order of width:

```
[Rectangle { width: 3, height: 5 }, Rectangle { width: 7, height: 12 }, Rectangle { width: 10, height: 1 }]
```

---

### Try it in the runner

Press **Run**. The output should match the line above.

The comments in `src/main.rs` suggest sorting by height instead. The order changes, because the closure now returns a different value for each rectangle.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Consider the following API:

```rust
/// Executes a function with a mutable reference to each element of a vector
fn for_each_mut<T, F: ___(&mut T)>(v: &mut Vec<T>, mut f: F) {
    for x in v.iter_mut() {
        f(x);
    }
}
```

Which of the following function traits is most appropriate to fill in the blank?

○ `Fn`
→ Not quite. `Fn` would work, but it's more restrictive than needed. The closure is allowed to change its input, so `FnMut` is the better choice.
◉ `FnMut`
→ Yes. `f` is called many times, so `FnOnce` is wrong. `FnMut` accepts both `Fn` and `FnMut` closures, so it's the least restrictive choice that works.
○ `FnOnce`
→ Not quite. `f` is called once for every element, so it has to be callable more than once.
````

````quiz
Consider the following API:

```rust
pub struct Analyzer<F> {
    postprocess: F
}

impl<F: ___(i32) -> i32> Analyzer<F> {
    fn process(&self, n: i32) -> i32 { /* .. */ }
    pub fn pipeline(&self, n: i32) -> i32 {
        let n = self.process(n);
        (self.postprocess)(n)
    }
}
```

Which of the following function traits is most appropriate to fill in the blank?

○ `FnMut`
→ Not quite. `pipeline` borrows `self` immutably, so the closure must be callable without changing anything. An `FnMut` closure might change its captures, and couldn't be
called here.
○ `FnOnce`
→ Not quite. `pipeline` can be called many times, so the closure can't be used up by one call.
◉ `Fn`
→ Yes. `pipeline` can be called many times, and it takes `&self`, so the closure must not change what it captures. That's exactly what `Fn` promises.
````

---

*For more detail, see section 13.1 "Closures: Anonymous Functions That Capture Their Environment" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch13-01-closures.html).*
