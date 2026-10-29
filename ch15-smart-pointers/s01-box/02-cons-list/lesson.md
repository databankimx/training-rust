## Enabling Recursive Types with Boxes

Some types contain values of the same type. A list is an example: each item holds the next one. Rust has to know how much space each type takes at compile time, and a type
that contains itself directly would need infinite space. A box solves this, because a box has a known size, whatever it points to.

The sample uses a *cons list*, a list built from nested pairs. Each `Cons` holds a value and the rest of the list, and the end of the list is `Nil`:

```rust,ignore
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

Without the `Box`, a `Cons` would hold a whole `List` inside itself, and the size would never end. The box stores the rest of the list on the heap, and the `Cons` holds only
a pointer, which has a fixed size.

A list is built from the inside out, with each `Cons` wrapping the next:

```rust,ignore
let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
```

The function `sum` walks the list. For each `Cons`, it adds the value to the sum of the rest, and for `Nil` it returns zero:

```rust,ignore
fn sum(list: &List) -> i32 {
    match list {
        Cons(value, rest) => value + sum(rest),
        Nil => 0,
    }
}
```

The program prints:

```
sum = 6
```

`sum(rest)` passes a `&Box<List>` where a `&List` is expected. Rust converts it automatically, because `Box<T>` implements `Deref`, which lets a box be used like the value
it holds. Chapter 15.2 covers `Deref` in detail.

---

### Try it in the runner

Press **Run**. The output should be `sum = 6`.

The comments in `src/main.rs` suggest removing the `Box` around the final `Nil`. The compiler rejects that, because the innermost `Cons` must hold a `Box<List>`, not a `List`.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Does this program compile? If it does, what does it print?

```rust
fn main() {
    let mut n = 1;
    let b = Box::new(&mut n);
    **b += 1;
    println!("{}", n);
}
```

○ This program does not compile
○ This program compiles, and prints `1`
◉ This program compiles, and prints `2`
→ Yes. `b` is a box holding a mutable reference to `n`. `*b` gives the reference, and `**b` gives `n` itself, so `+= 1` changes `n` to 2.
````

````quiz
Say we have a program with a variable:

```rust
let x: [Box<(usize, usize)>; 4] = /* ... */;
```

For a 64-bit target, what is the minimum possible size in bytes of `x` on the stack?

○ 8
○ 16
◉ 32
○ 64
→ Yes. Each box stores a pointer, which is 8 bytes on a 64-bit target, and the array holds four of them, so 4 × 8 = 32 bytes. The tuples themselves are on the heap.
````

---

*For more detail, see section 15.1 "Using Box<T> to Point to Data on the Heap" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch15-01-box.html).*
