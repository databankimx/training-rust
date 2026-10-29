## Creating and Using Iterators

An iterator produces a sequence of items one at a time. A vector can give you one with its `iter` method:

```rust,ignore
let v1 = vec![1, 2, 3];
let mut v1_iter = v1.iter();
```

Creating the iterator does nothing on its own. The items are produced only when something asks for them. Rust iterators are *lazy* in this way.

The `next` method asks for one item. It returns `Some` with the item, or `None` when the sequence has ended:

```rust,ignore
println!("{:?}", v1_iter.next());
```

Each call to `next` moves the iterator on, so the iterator must be declared `mut`. The sample calls `next` four times, and prints:

```
Some(1)
Some(2)
Some(3)
None
```

The fourth call returns `None`, because all three items have been used. The values are references into `v1`, because `iter` borrows the vector.

A `for` loop takes ownership of an iterator and calls `next` for you. That's why the loop doesn't need `mut`:

```rust,ignore
for value in v2_iter {
    println!("loop: {value}");
}
```

The loop prints `loop: 1`, `loop: 2`, and `loop: 3`.

Iterators also work with sequences that aren't vectors, which is why they're useful. The same `next` method is available on any type that implements the `Iterator` trait.

---

### Try it in the runner

Press **Run**. The output should match the seven lines above.

The comments in `src/main.rs` suggest changing `iter` to `into_iter`. The loop still compiles, but the values are now owned `i32`s, not references, and `v1` can't be used afterwards.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Which of the following best describes why iterators are described as "lazy"?

○ An iterator creates a copy of each item it iterates over
→ Not quite. Copying isn't what makes an iterator lazy, and iterators often don't copy at all.
○ An iterator takes ownership of the sequence of items it is iterating over
→ Not quite. Taking ownership is a separate question. Some iterators borrow, and some take ownership.
○ An iterator will only iterate over a given number of items
→ Not quite. An iterator can produce any number of items, so this isn't the reason.
◉ An iterator has no effect until you call methods that extract elements from the iterator
→ Yes. Creating an iterator does no work. The items are produced only when `next`, or a method that calls it, is used.
````

````quiz
True or false: these two code snippets are semantically equivalent, where `iter` is an iterator.

Snippet 1:
```rust
while let Some(x) = iter.next() {
    f(x);
}
```

Snippet 2:
```rust
for x in iter {
    f(x);
}
```

○ False
◉ True
→ Yes. A `for` loop is shorthand for a `while let` that calls `next` until it returns `None`, which is what the two snippets do.
````

---

*For more detail, see section 13.2 "Processing a Series of Items with Iterators" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch13-02-iterators.html).*
