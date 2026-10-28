## Using move Closures with Threads

A closure can use values from the code around it. By default, the closure borrows them, when that's enough. A thread is different, because the thread might outlive the code
that made the value. If the closure only borrowed the vector, the main thread could drop it while the spawned thread still used it, and the reference would point at nothing.

Rust rejects that. The compiler can't tell how long the thread will run, so it refuses a borrow that might not last. The `move` keyword changes the closure to take
ownership of the values it uses:

```rust,ignore
let v = vec![1, 2, 3];

let handle = thread::spawn(move || {
    println!("Here's a vector: {v:?}");
});

handle.join().unwrap();
```

Now the closure owns `v`, so the main thread can't use it afterward. The program prints:

```
Here's a vector: [1, 2, 3]
```

Without `move`, the compiler reports that the closure may outlive the current function while borrowing `v`. The fix the compiler suggests is the `move` keyword. The keyword doesn't make
it unsafe to share data. It transfers the value, so only one thread owns it.

---

### Try it in the runner

Press **Run**. The output should be `Here's a vector: [1, 2, 3]`.

The comments in `src/main.rs` suggest removing `move`. The compiler then reports the closure's borrow of `v` as a problem, because the thread might run after `v` is gone.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Does this program compile? If it does, what does it print?

```rust
use std::thread;
fn main() {
    let mut n = 1;
    let t = thread::spawn(move || {
        n = n + 1;
        thread::spawn(move || {
            n = n + 1;
        })
    });
    n = n + 1;
    t.join().unwrap().join().unwrap();
    println!("{n}");
}
```

○ This program does not compile
○ This program compiles, and prints `1`
◉ This program compiles, and prints `2`
○ This program compiles, and prints `3`
→ Yes. `move` copies `n` into each closure, so the increments inside the threads change their own copies, not the `n` in `main`. The main thread's `n` goes from 1 to 2 and prints 2.
````

````quiz
Consider this example, where a vector is captured by a thread without `move`:

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

The compiler includes this note: "function requires argument type to outlive `'static`". Which of the following best describes it?

○ The closure passed to `thread::spawn` is not allowed to take any arguments
○ The vector `v` contains values that do not outlive `'static`
○ A closure may only capture string constants of type `&'static str`
◉ Rust doesn't know how long a thread will run, so the thread's captures must live forever
→ Yes. `'static` means a value that lives for the whole program. A thread's closure has to satisfy that, because the thread could run for any length of time. A borrow of `v` can't promise that.
````

---

*For more detail, see section 16.1 "Using Threads to Run Code Simultaneously" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch16-01-threads.html).*
