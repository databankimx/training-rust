## Consuming and Adapting Iterators

Iterator methods fall into two groups. *Consuming adapters* call `next` until the iterator is finished, and use up the iterator. *Iterator adapters* take an
iterator and return a new one, without using it up.

### Consuming with sum

`sum` takes ownership of the iterator, adds up the items, and returns the total:

```rust,ignore
let v1_iter = v1.iter();
let total: i32 = v1_iter.sum();
```

After the call, `v1_iter` has been moved into `sum`, so it can't be used again. The sample prints `sum: 6`.

### Adapting with map and collect

`map` is an adapter. It takes a closure, and returns an iterator whose items are the closure's results:

```rust,ignore
let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
```

Creating the `map` iterator does nothing by itself, because iterators are lazy. `collect` is a consuming method, and it asks the `map` iterator for each item, and
gathers the results into a `Vec`. The type annotation `Vec<i32>` tells `collect` which collection to build. The sample prints `[2, 3, 4]`.

Adapters and consuming methods can be chained. Each adapter gives a new iterator, and the chain ends with a consuming method. Without the final `collect` or `sum`,
the closures would never run, and the compiler warns about that.

---

### Try it in the runner

Press **Run**. The output should be:

```
sum: 6
[2, 3, 4]
```

The comments in `src/main.rs` suggest calling `sum` on a fresh `v1.iter()`. Then think about why `v1_iter` can't be reused, and what the compiler would report
if it could.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Does this program compile? If it does, what does it print?

```rust
fn main() {
    let v = vec![1, 2, 3, 4];
    let a: Vec<_> = v.iter().filter(|x: &&i32| *x % 2 == 0).map(|x: &i32| x * 2).collect();
    let b: Vec<_> = v.iter().map(|x: &i32| x * 2).filter(|x: &i32| x % 2 == 0).collect();
    println!("{} {}", a[0], b[0]);
}
```

○ This program does not compile
→ Not quite. Both chains type-check. The first filter needs the extra `*`, because it receives `&&i32`, and the code already has it.
○ This program compiles, and prints `2 2`
→ Not quite. The `filter` in the first chain keeps the even numbers, and `map` doubles them. The first even number is 2, so the first item is 4, not 2.
◉ This program compiles, and prints `4 2`
→ Yes. In the first chain, `filter` keeps 2 and 4, and `map` doubles them, so `a[0]` is 4. In the second chain, `map` doubles first, so the first item is 2, which is even, and `b[0]` is 2.
○ This program compiles, and prints `4 4`
→ Not quite. The second chain doubles before filtering, and the first item it keeps is 2, not 4.
````

---

*For more detail, see section 13.2 "Processing a Series of Items with Iterators" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch13-02-iterators.html).*
