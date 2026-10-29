## Creating a Reference Cycle

Rust's ownership rules make a memory leak hard to create by accident, but they don't rule one out. A leak is possible with `Rc<T>` and `RefCell<T>` together: two values can
hold `Rc` references to each other, forming a *cycle*. Each value's reference count then stays above zero, so neither is ever dropped. This isn't unsafe, but it wastes
memory, and a program that builds many cycles can run out of it.

The sample builds a two-element cycle. The list `a` starts as `5, Nil`, and `b` is `10`, pointing to `a`. Then `a`'s tail is changed to point to `b`:

```rust,ignore
let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

if let Some(link) = a.tail() {
    *link.borrow_mut() = Rc::clone(&b);
}
```

The `RefCell` in each `Cons` is what allows the change, because `a` itself is shared and can't be modified directly. The program prints the counts as it goes:

```
a initial rc count = 1
a next item = Some(RefCell { value: Nil })
a rc count after b creation = 2
b initial rc count = 1
b next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })
b rc count after changing a = 2
a rc count after changing a = 2
```

After the change, `a` is owned by the variable `a` and by `b`'s tail, so its count is 2. `b` is owned by the variable `b` and by `a`'s tail, so its count is also 2.

At the end of `main`, Rust drops `b`, and then `a`. Each drop lowers a count by one, but each count is still 1, because the other value still holds a reference. Neither list is freed, and
the memory is lost for the rest of the program. This is a reference cycle, and it's a logic error in the program, not something the compiler catches.

---

### Try it in the runner

Press **Run**. The output should match the seven lines above.

The last `println!` is commented out, because printing the list from `a` follows the cycle forever. It would overflow the stack. Predict what it would print, and don't
uncomment it in the runner.

---

*For more detail, see section 15.6 "Reference Cycles Can Leak Memory" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html).*
