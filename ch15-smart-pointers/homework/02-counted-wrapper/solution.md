## Solution: A Counted Wrapper

This walkthrough explains the example solution in `src/main.rs`. Read it after you've written your own version.

### Run the solution

Press **Run** for the output, and **Test** for the test. The output is:

```
created first
created second
first has 5 bytes
second has 3 items
dropped second
dropped first
```

### How it works

`Logged::new` prints a message and builds the wrapper. The fields are the value, and a `name` that's used in the messages. The name is a `&'static str`, so it
lives for the whole program, and the wrapper doesn't need to own it.

`Deref` returns a reference to the inner value. Method calls such as `first.len()` work through it, because the compiler looks for `len` on the wrapper, and then on the
value that `deref` returns. This is the deref coercion from Chapter 15.2, applied to a type of our own.

`Drop` prints the name when the wrapper goes out of scope. The compiler inserts that call automatically at the end of `main`, as Chapter 15.3 described. The values are dropped
in reverse order of creation, so `second` is dropped before `first`, and the message order follows.

The test uses `*number`, which follows the deref to the inner `7`. It also prints the creation and drop messages, which appear in the test output.

### The extensions

Stopping `Drop` from running is not possible for a value that's still in its wrapper, because the wrapper's destructor always runs. A method that takes `self` by value can
move the inner value out, and the wrapper is then consumed. Rust doesn't allow moving a field out of a type that implements `Drop`, so the method must instead use a function such
as `std::mem::replace`, or construct the value through another route. `ManuallyDrop` is the standard tool for this, and it's the honest answer to the question.

For `DerefMut`, the wrapper implements it alongside `Deref`, returning `&mut T`. Then `first.push_str(" world")` works, because the compiler can reach the `String` through a mutable
deref. The same `Drop` still applies.

---

*This walkthrough accompanies the exercise in Chapter 15 of the course. It's not from the book.*
