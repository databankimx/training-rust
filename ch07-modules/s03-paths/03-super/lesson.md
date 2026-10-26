## Starting Relative Paths with super

A relative path can start from the parent module, using `super`. It's like `..` in a filesystem path, which means the parent directory.

`super` is useful when a module is closely tied to its parent. Suppose a module needs a function from the parent. Writing `super::name` means the
module doesn't need to know where the parent sits in the tree. If the parent is later moved, the path still works.

---

### The sample

The sample models a chef who cooks a corrected order and then has it delivered. The function `deliver_order` belongs to the crate root. The
`back_of_house` module calls it with `super`:

```rust,ignore
fn deliver_order() {
    println!("Delivered the order");
}

mod back_of_house {
    fn cook_order() {
        println!("Cooked a new order");
    }

    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
}
```

Inside `back_of_house`, `super` refers to the module above it, which is the crate root. From there, the path finds `deliver_order`. A child module can
use private items from its ancestors, so the function doesn't need `pub` for this call.

The output is:

```
Cooked a new order
Delivered the order
```

---

### Why super rather than crate

Both `super::deliver_order` and `crate::deliver_order` work in this sample. The difference is what happens when the code moves. If `back_of_house` and
`deliver_order` are moved together, `super` keeps working. `crate` would break, because it always points at the root. Choose `super` when two items belong
together, and `crate` when a path should always point at the same place.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Cooked a new order
Delivered the order
```

The comments in `src/main.rs` suggest replacing `super::deliver_order()` with `crate::deliver_order()`. The output is the same here, because
`back_of_house` sits directly under the crate root.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Does this program compile? If it does, what does it print?

```rust
pub mod a {
  pub mod b {
    pub fn f() { println!("b1"); }
    pub mod c {
      pub fn f() { println!("c1"); }
    }
  }
  pub fn entry() { super::b::c::f(); }
}

pub mod b {
  pub fn f() { println!("b2"); }
  pub mod c {
    pub fn f() { println!("c2"); }
  }
}

fn main() {
  crate::a::entry();
}
```

○ This program does not compile
→ Not quite. The path `super::b::c::f` is valid from inside `a`.
◉ This program does compile
→ Yes. Inside `a`, `super` is the crate root, so `super::b` is the outer `b`, not the `b` inside `a`.
---
If it compiles, what does it print?

[ c2 ]
→ Yes. The path goes to the outer `b`, then to its child `c`, and calls `f`, which prints `c2`.
````

---

*For more detail, see section 7.3 "Paths for Referring to an Item in the Module Tree" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html).*
