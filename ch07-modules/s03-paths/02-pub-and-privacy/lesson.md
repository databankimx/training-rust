## Pub and Privacy

Items are private to their parent module by default. A function, struct, enum, module, or constant defined inside a module can be used from
the module that contains it, and from modules inside it. Code outside the module can't reach it unless it's marked `pub`.

This is the rule that makes Rust code easier to change. Whatever you don't mark `pub` is an implementation detail, and you can change it without
breaking code that depends on the module.

---

### Private inside, public outside

The sample has a public module, `foo`, with a child module `bar`. Inside `foo`, the function `a` calls `bar::b`:

```rust,ignore
pub mod foo {
    pub mod bar {
        pub fn b() {
            println!("b");
        }
    }

    pub fn a() {
        bar::b();
    }
}

fn main() {
    foo::a();
}
```

The call `bar::b()` is a relative path from `foo`, and it's allowed, because `a` sits in the same module as `bar`. `main` calls `foo::a`, which is
`pub`, and `a` does the rest of the work. The output is:

```
b
```

`main` never names `bar`. It doesn't need to, because `a` is the public entry point.

---

### What goes wrong without pub

If `bar` isn't marked `pub`, `main` can't name it, and a call like `foo::bar::b()` from `main` is rejected. The book's quiz shows this case: the
program fails on line 9, the call from `main`, because `bar` is private to `foo`. The `pub` on `b` isn't enough, because the path to `b` still goes
through the private module.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
b
```

The comments in `src/main.rs` suggest removing `pub` from `mod bar`, and calling `foo::bar::b()` from `main`. The compiler rejects the call, because
`bar` is private.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Does this program compile?

```rust
pub mod foo {
  fn a() { println!("a"); }
  mod bar {
    pub fn b() { println!("b"); }
  }
}

fn main() {
  foo::bar::b();
}
```

○ This program does compile
→ Not quite. The module `bar` is private to `foo`, so `main` can't reach `b` through it.
◉ This program does not compile
→ Yes. `b` is public, but the module `bar` that contains it is not, so the path is blocked.
---
On which line does Rust report the problem?

[ 9 ]
→ Yes. Line 9 is the call in `main`, which goes through the private module `bar`.
````

---

*For more detail, see section 7.3 "Paths for Referring to an Item in the Module Tree" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html).*
