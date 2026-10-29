## Performance of Code Using Generics

A generic type doesn't cost anything at runtime. The compiler does the work before the program runs, and the result is as fast as code written by hand
for each type.

The process is called *monomorphization*. The compiler finds each concrete type a generic definition is used with, and generates a separate, specific
version for each one.

---

### The example

The sample uses the standard library's `Option<T>` with two types:

```rust,ignore
let integer = Some(5);
let float = Some(5.0);
```

`integer` is an `Option<i32>`, and `float` is an `Option<f64>`. The compiler sees both, and produces two definitions, one for each type. Conceptually, the
result looks like two separate enums:

```rust,ignore
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}
```

The generic definition is replaced by these specific ones, and the program runs as if they had been written by hand. The sample's output is:

```
Some(5) Some(5.0)
```

---

### What this means for you

You can write one generic definition, and use it with as many types as you need, without paying for the flexibility at runtime. The cost is paid at compile
time, in the size of the compiled program. Each type a generic is used with produces its own copy of the code.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Some(5) Some(5.0)
```

The comments in `src/main.rs` suggest adding a third use, such as `Some('c')`. The compiler then produces a third version of `Option`, one for `char`.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Imagine using a third-party function whose implementation you don't know, but whose type signature is this:

```rust
fn mystery<T>(x: T) -> T {
  // ????
}
```

Then you call `mystery` like this: `let y = mystery(3);`

Assuming `mystery` uses no unsafe code, what must the value of `y` be?

○ 0
→ Not quite. `mystery` has no way to make a number out of nothing, because it knows nothing about `T`.
○ 1
→ Not quite. The function can't produce a value of its own, because `T` could be any type.
◉ 3
→ Yes. The only function with this signature, without unsafe code, returns its input unchanged. Since `mystery` knows nothing about `T`, it can't create a new value of that type.
○ It cannot be determined
→ Not quite. The signature constrains the answer enough to know it.
````

````quiz
Does this program compile? If not, on which line does Rust report the problem?

```rust
fn print_slice<T>(v: &[T]) {
  for x in v {
    println!("{x}");
  }
}

fn main() {
  print_slice(&[1, 2, 3]);
}
```

○ This program compiles
→ Not quite. `T` could be any type, and not all types can be printed.
◉ This program does not compile, on line 3
→ Yes. `T` might not implement `Display`, so `println!` can't print `x`. The generic function must say which capabilities `T` has, as Section 10.2 explains.
○ This program does not compile, on line 7
→ Not quite. Line 7 is the start of `main`, which is fine. The problem is the `println!` on line 3, since `T` has no known capabilities.
````

---

*For more detail, see section 10.1 "Generic Data Types" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch10-01-syntax.html).*
