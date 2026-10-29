## Running Code on Cleanup with the Drop Trait

The `Drop` trait lets a type run code when one of its values goes out of scope. You implement `drop`, which takes `&mut self`, and Rust calls it automatically. Cleanup
happens in the same place every time, so you don't need to remember to add it. This is how a file, a network connection, or a lock is released when it's no longer needed.

Smart pointers are where `Drop` matters most. When a `Box<T>` is dropped, the heap memory it points to is freed. You can add the same behavior to your own types.

This sample has a type that prints when it's dropped, so you can see when Rust runs the code:

```rust,ignore
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
```

In `main`, two values are created, and then a message is printed:

```rust,ignore
let _c = CustomSmartPointer { data: String::from("my stuff") };
let _d = CustomSmartPointer { data: String::from("other stuff") };
println!("CustomSmartPointers created.");
```

The names start with an underscore. The values are never used after they're created, and the underscore tells the compiler that's intended, so there's no warning. It
doesn't change when the values are dropped. A name that begins with an underscore still keeps its value alive until the end of the scope, which a lone `_` doesn't do.

At the end of `main`, Rust drops the values automatically. Variables are dropped in the reverse order of their creation, so `_d` is dropped before `_c`. The output is:

```
CustomSmartPointers created.
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```

Nobody called `drop` explicitly. The compiler inserted the call at the end of the scope.

---

### Try it in the runner

Press **Run**. The output should match the three lines above.

The comments in `src/main.rs` suggest swapping the two declarations. The values are then dropped in the opposite order, because the last value created is dropped first.

---

*For more detail, see section 15.3 "Running Code on Cleanup with the Drop Trait" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch15-03-drop.html).*
