## Watching the Reference Count Change

`Rc::strong_count` returns the number of owners of an `Rc` value. Calling it shows how the count moves as clones are made and as owners go out of scope:

```rust,ignore
let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
println!("count after creating a = {}", Rc::strong_count(&a));

let _b = Cons(3, Rc::clone(&a));
println!("count after creating b = {}", Rc::strong_count(&a));

{
    let _c = Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
}
println!("count after c goes out of scope = {}", Rc::strong_count(&a));
```

The program prints:

```
count after creating a = 1
count after creating b = 2
count after creating c = 3
count after c goes out of scope = 2
```

Each `Rc::clone` raises the count by one. When `_c` leaves its block, the count drops by one. No function call does that: the `Drop` implementation of `Rc` lowers the count
automatically, which is what you saw with `Drop` in the last sub-section. When the count reaches zero, the value is freed. Here `a` and `_b` are still alive at the end of
`main`, so the list isn't freed until both have gone.

The underscore in `_b` and `_c` stops the unused-variable warning. Each one still counts as an owner while it's in scope.

The `#[allow(dead_code)]` line above `List` is needed because this step never reads the list's fields. It only counts owners, so the compiler would otherwise warn that the fields are unused.

`Rc<T>` gives shared ownership, but only for reading. Changing the data through an `Rc` isn't allowed, because two owners could change it at the same time. The next
sub-section covers `RefCell<T>`, which allows changes with the rules checked at runtime.

---

### Try it in the runner

Press **Run**. The output should match the four lines above.

The comments in `src/main.rs` suggest adding a second clone inside the inner block. The count then reads one higher inside the block, and returns to 2 when the block ends.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Does this program compile? If not, on which line does Rust report the problem?

```rust
use std::rc::Rc;
fn main() {
    let n = Rc::new(1);
    let mut n2 = Rc::clone(&n);
    *n2 += 1;
    println!("{}", n);
}
```

○ This program compiles
○ This program does not compile, on line 4
◉ This program does not compile, on line 5
→ Yes. Data inside an `Rc` can't be changed directly, because `Rc` implements `Deref` but not `DerefMut`. The change on line 5 is rejected.
````

````quiz
Consider this program, where `Example` prints `drop` when it's dropped:

```rust
use std::rc::Rc;
struct Example;
impl Drop for Example {
    fn drop(&mut self) {
        println!("drop");
    }
}
fn main() {
    let x = Rc::new(Example);
    let y = Rc::clone(&x);
    println!("A");
    drop(x);
    println!("B");
    drop(y);
    println!("C");
}
```

What does it print?

○ `A`, `drop`, `B`, `C`
◉ `A`, `B`, `drop`, `C`
○ `drop`, `A`, `B`, `C`
○ `A`, `B`, `C`, `drop`
→ Yes. Dropping `x` lowers the count from two to one, and `y` still owns the value, so nothing is freed. Dropping `y` brings the count to zero, which runs `drop` before `C`.
````

---

*For more detail, see section 15.4 "Rc<T>, the Reference-Counted Smart Pointer" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch15-04-rc.html).*
