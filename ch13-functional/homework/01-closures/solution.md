## Solution: Closures

This walkthrough explains the example solution in `src/main.rs`. Read it after you've written your own version.

### Run the solution

Press **Run** for the output, and **Test** for the tests. The output is:

```
counter: 1 2 3
apply_twice: 18
sorted: ["quick", "brown", "jumps", "fox"]
```

### How it works

`make_counter` has a local variable, `count`, and returns a closure that increases it. The `move` keyword gives the closure ownership of `count`. Without `move`, the
closure would borrow `count`, which goes out of scope when the function returns, so the compiler would reject the code. The return type `impl FnMut() -> u32` says the
closure changes its own state, so it has to be called through a `mut` binding.

`apply_twice` takes any closure with the type `Fn(i32) -> i32`. It calls `f` on `x`, then on the result. The bound is `Fn`, which is the least demanding trait that
works, because `f` is only called, never changed. A closure that changes its captured values would need `FnMut` instead.

`sort_by_length` passes `Reverse(word.len())` as the key. `Reverse` wraps a value and reverses its ordering, so the longest word sorts first. The sort is stable, so
words of equal length keep their original order.

In `main`, `(0..3).map(|_| counter().to_string())` calls the counter three times. The closure passed to `map` captures `counter` mutably, and `map` accepts it because
it's `FnMut`.

### The extensions

`apply_twice` could take a count, and loop that many times. The closure would still be `Fn`, because it only reads its input. Only a closure that changes what it captures
would need `FnMut`.

`make_adder(n: i32) -> impl Fn(i32) -> i32` returns a closure that uses `n`. Without `move`, the closure would borrow `n`, which is a parameter that disappears when the
function returns, so the code won't compile. With `move`, the closure owns its copy of `n`.

---

*This walkthrough accompanies the exercise in Chapter 13 of the course. It's not from the book.*
