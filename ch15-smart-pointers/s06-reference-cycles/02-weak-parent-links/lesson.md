## Preventing Cycles with Weak<T>

Some structures naturally have links in both directions. A tree is one: a parent holds its children, and each child knows its parent. If both directions were `Rc` references,
each would keep the other alive, and the cycle would leak. The fix is to split the relationship by ownership. The parent owns its children, so it holds them with `Rc`.
The child only refers to its parent, so it holds a `Weak<T>`.

`Weak<T>` is a reference that doesn't count toward ownership. It's created with `Rc::downgrade`, and it's checked with `upgrade`, which returns `Some` if the value still exists,
and `None` once it's been dropped. Because the value might be gone, the program has to handle both cases.

```rust,ignore
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
```

The sample builds a parent, `branch`, and its child, `leaf`, inside a block. Once the block ends, `branch` is dropped, and the child's link to it is no longer valid. The program
prints the counts and the parent's value at each step:

```
leaf strong = 1, weak = 0
branch strong = 1, weak = 1
branch children = 1
leaf strong = 2, weak = 0
leaf parent value = Some(5)
leaf parent value after scope = None
leaf strong = 1, weak = 0
```

The weak count on `branch` is 1, because `leaf` holds a weak reference to it. That count doesn't stop `branch` from being dropped, which is why the strong count reaches zero at the end
of the block, and the value is freed. Once it is, `upgrade` returns `None`.

`leaf` shows the other side. Its strong count is 2 inside the block, because `branch` holds it in `children`, and 1 after the block, when only the variable `leaf` is left.

---

### Try it in the runner

Press **Run**. The output should match the seven lines above.

The comments in `src/main.rs` suggest changing the value of `branch`. Predict what the parent value prints inside the block before you run it.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Consider this program. What does it print?

```rust
use std::rc::Rc;
fn main() {
    let r1 = Rc::new(0);
    let r4 = {
        let r2 = Rc::clone(&r1);
        Rc::downgrade(&r2)
    };
    let r5 = Rc::clone(&r1);
    let r6 = r4.upgrade();
    println!("{} {}", Rc::strong_count(&r1), Rc::weak_count(&r1));
}
```

○ `2 1`
◉ `3 1`
○ `3 2`
○ `2 2`
→ Yes. The strong references are `r1`, `r5`, and `r6`, which makes 3. `r2` was dropped at the end of its block, and `r4` is the only weak reference, which makes 1 weak count. Calling `upgrade` on `r4` gives `r6` a strong reference, which is why `r6` counts.
````

---

*For more detail, see section 15.6 "Reference Cycles Can Leak Memory" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html).*
