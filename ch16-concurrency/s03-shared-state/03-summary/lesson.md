## Summary: Shared State

Shared-state concurrency gives several threads access to the same data, and the data must be guarded.

* **Mutex<T>:** only one thread holds the lock at a time, and the data is reached only through `lock`. The guard releases the lock when it goes out of scope. *(See the 16.3 steps.)*
* **Arc<T>:** lets several threads own the same value, with a thread-safe count. `Rc<T>` isn't safe across threads, so the compiler refuses it. *(See the shared counter step.)*
* **Together:** `Arc<Mutex<T>>` is the standard shape for shared, changeable state across threads. *(See the shared counter step.)*
* **Limits:** Rust prevents data races, but not deadlocks. Locking two mutexes in different orders can leave threads waiting for each other forever. *(See the mutex step.)*

Message passing and shared state solve the same problem in different ways. Channels move ownership of each value to one receiver. A mutex keeps one copy of the data, and lets any
thread change it while it holds the lock.

---

### Check your understanding

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
In some concurrency APIs, a mutex is separate from the data it guards. For example, imagine a hypothetical mutex API like this:

```
let mut data = Vec::new();
let mx: Mutex = Mutex::new();
{
    let _guard = mx.lock();
    data.push(0);
}
```

Which of the following best describes why Rust uses `Mutex<T>` instead of just `Mutex`?

◉ To prevent accessing a mutex's data without locking the mutex
○ To require fewer calls to mutex methods
○ To improve the efficiency of concurrent programs with mutexes
○ To prevent a mutex's data from being moved between threads
→ Yes. In `Mutex<T>` the data lives inside the mutex, so the only way to reach it is through `lock`. A separate mutex and data would let code change the data without holding the lock.
````

````quiz
Consider this program, which shares an `Arc` containing a reference to a `String`:

```rust
use std::{sync::Arc, thread};

fn main() {
    let s = String::from("Hello world");
    let a = Arc::new(&s);
    let a2 = Arc::clone(&a);
    let t = thread::spawn(move || a2.len());
    let len = t.join().unwrap();
    println!("{} {}", a, len);
}
```

Does this program compile? If not, on which line does Rust report the problem?

○ This program compiles, and prints `Hello world 11`
○ This program does not compile, on line 6
◉ This program does not compile, on line 7
○ This program does not compile, on line 8
→ Yes. The closure passed to `thread::spawn` must be `'static`, and an `Arc` holding a reference to `s` can't promise that. The problem is reported on the `thread::spawn` call on line 7.
````

---

*The summary is written for this course, and draws on section 16.3 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch16-03-shared-state.html).*
