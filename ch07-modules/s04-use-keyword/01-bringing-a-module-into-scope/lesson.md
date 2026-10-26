## Bringing Paths into Scope with use

Writing out a full path every time you call a function is repetitive. The `use` keyword creates a shortcut: you write the path once, and then
use a shorter name in that scope.

---

### The sample

The sample brings the `hosting` module into scope with `use`, and then calls a function through it:

```rust,ignore
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

After the `use`, the name `hosting` is valid in that scope, as if the module had been defined there. The call `hosting::add_to_waitlist()` is
shorter than the full path, but reaches the same function. The output is:

```
Added to waitlist
```

`use` is like a symbolic link in a filesystem. It doesn't copy anything, and it doesn't change what the path refers to. It only gives the path a
shorter name.

---

### Use also checks privacy

A `use` path is subject to the same privacy rules as any other path. The module `hosting` is `pub`, so the `use` works. If it weren't, the `use`
itself would be rejected.

---

### A use applies only in its own scope

A `use` shortcut applies only in the scope where it's written. Moving `eat_at_restaurant` into a child module would break the call, because the
shortcut doesn't come along. Within that child module, you'd need another `use`, or a `super::` path to the parent's shortcut.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Added to waitlist
```

The comments in `src/main.rs` suggest removing the `use` line and writing the full path in `eat_at_restaurant`. The output is the same. The `use`
changes how the code reads, not what it does.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Which of the following statements best describes the function of the `use` keyword?

○ `use` copies the contents of an item from one module to another
→ Not quite. `use` doesn't copy anything. It creates a shorter name for an existing path.
○ `use` allows access to items that circumvents normal privacy rules
→ Not quite. A `use` path is subject to the same privacy rules as any other path.
○ `use` indicates to the compiler that an item will be used and should be optimized
→ Not quite. `use` has no effect on optimization.
◉ `use` reduces the verbosity of referring to items in the used path
→ Yes. It creates a shorter name for a path, so later references in the scope are shorter.
````

---

*For more detail, see section 7.4 "Bringing Paths into Scope with the use Keyword" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html).*
