## Dropping a Value Early with std::mem::drop

Rust won't let you call the `drop` method of the `Drop` trait yourself. The compiler inserts that call at the end of each scope, and a second call would free the value twice.
If you need a value cleaned up early, use the function `std::mem::drop`, which is in the prelude. It takes the value by moving it in, and the value is dropped inside the function:

```rust,ignore
drop(c);
```

This sample creates two values, drops one of them early, and prints a message:

```rust,ignore
let c = CustomSmartPointer { data: String::from("some data") };
let _d = CustomSmartPointer { data: String::from("other stuff") };
println!("CustomSmartPointers created.");

drop(c);
println!("CustomSmartPointer dropped before the end of main.");
```

The output is:

```
CustomSmartPointers created.
Dropping CustomSmartPointer with data `some data`!
CustomSmartPointer dropped before the end of main.
Dropping CustomSmartPointer with data `other stuff`!
```

The value `c` is dropped at the call to `drop`, so its message comes before the one that says it's been dropped. The value `_d` is still alive, and it's dropped at the
end of `main`, as before. This is useful when a lock or a file should be released before the rest of a function runs.

---

### Try it in the runner

Press **Run**. The output should match the four lines above.

The comments in `src/main.rs` suggest calling `c.drop()` directly. The compiler rejects that, and says that explicit destructor calls aren't allowed.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Does this program compile? If not, on which line does Rust report the problem?

```rust
struct Example(i32);
impl Drop for Example {
    fn drop(&mut self) {
        self.0 += 1;
        println!("drop {}", self.0);
    }
}
fn main() {
    let e = Example(0);
    drop(e);
    drop(e);
}
```

○ This program compiles
○ This program does not compile, on line 10
◉ This program does not compile, on line 11
→ Yes. The first `drop(e)` moves `e` into the function. The second call on line 11 uses a value that has already been moved and dropped, so the compiler rejects it.
````

````quiz
Consider this snippet, which creates a string:

```rust
fn main() {
    let mut s = String::new();
    ____
}
```

Which of these fills the blank in a way that is NOT valid?

○ `drop(s);`
○ `(|_| ())(s);`
○ `{ s };`
◉ `s.drop();`
→ Yes. Each of the others moves `s` and discards it, so it's dropped. `s.drop()` is an explicit call to the `Drop` trait's method, which Rust doesn't allow.
````

---

*For more detail, see section 15.3 "Running Code on Cleanup with the Drop Trait" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch15-03-drop.html).*
