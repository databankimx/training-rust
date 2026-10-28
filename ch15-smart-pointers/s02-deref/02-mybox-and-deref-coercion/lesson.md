## Defining Our Own Smart Pointer, and Deref Coercion

A `Box<T>` can be followed with `*`, but a type of your own can't, unless it implements `Deref`. This step builds `MyBox<T>`, a wrapper that works like `Box<T>` for
this purpose. It doesn't store its data on the heap, which doesn't matter here. The point is the pointer-like behavior.

```rust,ignore
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

`Deref` has one method, `deref`, which returns a reference to the inner value. When you write `*y`, Rust calls `deref` and then follows the reference, so `*y` gives the
value inside. The `type Target = T;` line says what the reference points to.

The sample checks that `*y` gives `5`, and prints it:

```
*y = 5
```

---

### Deref coercion

Deref coercion is the other half of the feature. A function that takes a `&str` can be called with a `&MyBox<String>`:

```rust,ignore
let m = MyBox::new(String::from("Rust"));
hello(&m);
```

Rust converts the reference automatically. It calls `deref` to get a `&String`, and then `String`'s own `Deref` to get a `&str`. Each step is inserted by the compiler, so
the call needs no extra `&` or `*` symbols, and there's no runtime cost. The output is:

```
Hello, Rust!
```

Without deref coercion, you would write `hello(&(*m)[..])`, which is much harder to read.

---

### Try it in the runner

Press **Run**. The output should be `*y = 5` followed by `Hello, Rust!`.

The comments in `src/main.rs` suggest removing the `Deref` implementation. The compiler then stops at `*y`, because `MyBox` can't be dereferenced, and it stops at
`hello(&m)`, because the coercion has nothing to use.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Consider this program, where `AccessLogger` prints `deref` every time it's dereferenced:

```rust
use std::ops::Deref;

#[derive(Clone, Copy)]
struct AccessLogger(i32);

impl Deref for AccessLogger {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        println!("deref");
        &self.0
    }
}

fn main() {
  let n = AccessLogger(-1);
  let x = *n + 1;
  let n2 = n;
  println!("{} {}", x, *n)
}
```

What does it print?

○ `0 -1`
○ `deref` and then `0 -1`
◉ `deref`, `deref`, and then `0 -1`
○ The program does not compile
→ Yes. `*n` appears twice, and each use calls `deref`, which prints a line. Copying `n` into `n2` doesn't dereference it, so it prints nothing.
````

---

*For more detail, see section 15.2 "Treating Smart Pointers Like Regular References with the Deref Trait" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch15-02-deref.html).*
