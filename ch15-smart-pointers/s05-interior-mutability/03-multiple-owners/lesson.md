## Multiple Owners of Mutable Data

`Rc<T>` lets several owners share a value, but it only allows reading. `RefCell<T>` allows changes, but has one owner. Combining them gives a value that many owners
can share and change: `Rc<RefCell<T>>`.

The sample uses the cons list from Chapter 15.4, and makes its first value shared and changeable:

```rust,ignore
let value = Rc::new(RefCell::new(5));
let a = Rc::new(Cons(
    Rc::clone(&value),
    Rc::new(Cons(Rc::new(RefCell::new(10)), Rc::new(Nil))),
));
let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
```

`value` is cloned into `a`, so both `value` and `a` refer to the same `RefCell`, holding 5. Later, `*value.borrow_mut() += 10` changes that one number, and every list
that reaches it sees the change. The `sum` function reads each value through `borrow`:

```
before: a = 15, b = 18, c = 19
after: a = 25, b = 28, c = 29
```

Before the change, `a` sums to 5 + 10 = 15. After it, the shared value is 15, so `a` sums to 15 + 10 = 25. The lists `b` and `c` both pass through `a`, so they change too, and
their own values, 3 and 4, are added on top.

The outside appears immutable. `a`, `b`, and `c` are all declared without `mut`, but the contents can still be changed, because the change happens through `RefCell`, under
its runtime checks.

`RefCell<T>` doesn't work in code that runs on several threads. The thread-safe version is `Mutex<T>`, which Chapter 16 covers.

---

### Try it in the runner

Press **Run**. The output should match the two lines above.

The comments in `src/main.rs` suggest adding 100 to `value`. Work out the three new sums first. Each list's sum should go up by the same amount, because they all share `value`.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Which of the following best describes the concept of interior mutability in Rust?

○ Wrapping `unsafe` code in a safe API
○ Allowing data on the inside of a data structure to be mutated
◉ Allowing data to be mutated through an immutable reference
○ Using the borrow checker to enforce memory safety at runtime
→ Yes. Interior mutability lets a value be changed through a reference that looks immutable. The unsafe code inside the type is what makes that possible.
````

````quiz
Consider an API that tracks the number of calls to a particular method:

```rust
struct Api {
    count: ???
}

impl Api {
    fn some_method(&self) {
        // increment count
        // rest of the method...
    }
}
```

Say the count is a `usize`. Which wrapper type is the most appropriate for this situation?

◉ `RefCell<usize>`
○ `Rc<usize>`
○ `Box<usize>`
○ None, `usize` is fine
→ Yes. `some_method` takes `&self`, so the count can't be changed directly. `RefCell<usize>` lets the method increment it through an immutable reference. For a plain integer like this,
`Cell<usize>` would also work, and is lighter.
````

````quiz
Consider this incorrect implementation of a `RefCell`, which doesn't check whether its value is already borrowed:

```rust
use std::cell::UnsafeCell;
struct BadRefCell<T>(UnsafeCell<T>);
impl<T> BadRefCell<T> {
    pub fn borrow_mut(&self) -> &mut T {
        unsafe { &mut *self.0.get() }
    }
}
```

Given `let v = BadRefCell(UnsafeCell::new(vec![1, 2, 3]));`, which snippet would violate memory safety?

◉ `let v1 = v.borrow_mut(); let n = &v1[0]; v.borrow_mut().push(0); println!("{n}");`
○ `drop(v.borrow_mut()); drop(v.borrow_mut());`
○ `let v1 = v.borrow_mut(); let v2 = v.borrow_mut(); v1.push(0); v2.push(0);`
○ `v.borrow_mut().push(0); let n = v.borrow_mut()[0]; println!("{n}");`
→ Yes. `n` refers to an element of the vector. The `push` may reallocate the vector, which would leave `n` pointing at freed memory. The real `RefCell` prevents this, because the second
`borrow_mut` would panic while `v1` is still active.
````

---

*For more detail, see section 15.5 "RefCell<T> and the Interior Mutability Pattern" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html).*
