## Capturing References or Moving Ownership

A closure captures values from its environment in one of three ways, matching the ways a function can take a parameter: by immutable reference, by mutable reference,
or by taking ownership. The closure chooses the least it needs, based on what its body does with each value.

The sample has two closures. The first only reads `list`, so it captures an immutable reference:

```rust,ignore
let list = vec![1, 2, 3];
let only_borrows = || println!("From closure: {list:?}");
```

Because `list` is only borrowed immutably, the program can still read it before and after the closure is defined and called.

The second closure changes `list`, so it captures a mutable reference:

```rust,ignore
let mut list = vec![1, 2, 3];
let mut borrows_mutably = || list.push(7);
borrows_mutably();
```

Here the closure has to be declared `mut`, because calling it changes the captured value. While the closure holds the mutable borrow, nothing else can use `list`.
Once the closure's last call has happened, the borrow ends.

The program prints:

```
Before defining closure: [1, 2, 3]
Before calling closure: [1, 2, 3]
From closure: [1, 2, 3]
After calling closure: [1, 2, 3]
Before defining closure: [1, 2, 3]
After calling closure: [1, 2, 3, 7]
```

---

### Moving ownership with move

To force a closure to take ownership of a value, even when its body doesn't need ownership, put the `move` keyword before the parameter list. This matters most when a
closure is sent to another thread, because the thread can outlive the code that made the value. Chapter 16 covers threads, so the example isn't in this sample.

---

### Try it in the runner

Press **Run**. The output should match the six lines above.

The comments in `src/main.rs` suggest adding a `println!` between defining `borrows_mutably` and calling it. The compiler rejects that, because no other use of `list`
is allowed while the mutable borrow is active.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Does this program compile? If it does, what does it print? If not, on which line does Rust report the problem?

```rust
fn main() {
    let mut s = String::from("Hello");
    let mut add_suffix = || s.push_str(" world");
    println!("{s}");
    add_suffix();
}
```

○ This program compiles, and prints `Hello`
→ Not quite. The closure borrows `s` mutably, and `println!` reads `s` while that borrow is still active.
○ This program compiles, and prints `Hello world`
→ Not quite. The closure isn't called until after the `println!`, and the program doesn't compile anyway.
◉ This program does not compile, on line 5
→ Yes. `add_suffix` holds a mutable borrow of `s` from its definition to its call, so reading `s` in the `println!` is invalid.
○ This program does not compile, on line 4
→ Not quite. Line 4 is the definition of the closure, which is allowed. The problem is the read on line 5.
````

````quiz
Does this program compile? If it does, what does it print?

```rust
fn main() {
    let mut s = String::from("Hello");
    let add_suffix = |s: &mut String| s.push_str(" world");
    println!("{s}");
    add_suffix(&mut s);
}
```

○ This program does not compile
→ Not quite. The closure takes `s` as a parameter, so it doesn't capture the outer `s`, and the program is valid.
◉ This program compiles, and prints `Hello`
→ Yes. Because the closure doesn't capture `s`, the `println!` can read it before the closure changes it.
○ This program compiles, and prints `Hello world`
→ Not quite. The `println!` runs before the closure is called, so `s` is still `Hello` when it's printed.
````

---

*For more detail, see section 13.1 "Closures: Anonymous Functions That Capture Their Environment" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch13-01-closures.html).*
