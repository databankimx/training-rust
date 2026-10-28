## Type Inference with Trait Objects

Rust infers the types of values from how they're used, so an annotation is often unnecessary. Trait objects are a case where it's needed. This step pulls the `components` vector out into
its own variable, which is the kind of refactor that breaks the code:

```rust,ignore
fn main() {
    let components = vec![
        Box::new(SelectBox { /* ... */ }),
        Box::new(Button { /* ... */ }),
    ];
    let screen = Screen { components };
    screen.run();
}
```

The compiler reports a type mismatch, with an error like this one:

```text
error[E0308]: mismatched types
    |
    |     Box::new(Button { ... }),
    |     ^^^^^^^^^^^^^^^^^^^^^^^^ expected `SelectBox`, found `Button`
```

In the first version, `Screen` said the vector must hold `Box<dyn Draw>`, so the compiler knew the type of every element. Once the vector is a separate variable, the compiler sees only the first
element, which is a `SelectBox`, and then rejects the `Button`. The information from the struct definition is lost at that point.

There are two ways to give the compiler the hint it needs. One is to cast a single element:

```rust,ignore
let components = vec![
    Box::new(SelectBox { /* ... */ }) as Box<dyn Draw>,
    Box::new(Button { /* ... */ }),
];
```

The other is a type annotation on the variable, which is what this step uses:

```rust,ignore
let components: Vec<Box<dyn Draw>> = vec![
    Box::new(SelectBox { /* ... */ }),
    Box::new(Button { /* ... */ }),
];
```

The output is the same as before:

```
Drawing a 75x10 select box with 3 options
Drawing a 50x10 button labeled 'OK'
```

Trait objects can make the code harder to use in this way, so it's worth writing the annotation where a vector of trait objects is built.

---

### Dynamic dispatch

When `run` calls `component.draw()`, the compiler can't know which `draw` will run, because the vector can hold any type that implements `Draw`. The program looks up the method at runtime,
through a table that the trait object carries. This is *dynamic dispatch*.

With generics, the compiler generates a separate version of each function for each concrete type, and knows the method at compile time. That's *static dispatch*, and it's usually faster,
because the compiler can inline the code. Dynamic dispatch costs a lookup on every call, and it stops the compiler from inlining. Rust accepts that cost to allow mixed types in one collection.

---

### Try it in the runner

Press **Run**. The output should match the two lines above.

The comments in `src/main.rs` suggest removing the annotation on `components`. The compiler then reports the same mismatched-types error as the book, and points at the `Button`.

---

*For more detail, see section 18.2 "Using Trait Objects That Allow for Values of Different Types" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch18-02-trait-objects.html).*
