## Solution: A Function Pipeline

This walkthrough explains the example solution in `src/main.rs`. Read it after you've written your own version.

### Run the solution

Press **Run** for the output, and **Test** for the three tests. The output is:

```
fn pipeline: 11
reversed: 12
boxed: 25
```

### How it works

Both runners use `fold`. The starting value is the accumulator, and each step takes the value so far and returns the next one. The order of the steps is the order of the slice, so the
pipeline runs from left to right.

`run_fns` takes `&[fn(i32) -> i32]`. Each named function, such as `double`, coerces to the `fn` type, so the slice can list them directly, as in `&[double, increment]`. These steps can't capture values.

`run_boxed` takes `&[Box<dyn Fn(i32) -> i32>]`. Each step is a trait object, so the steps can be different closures, and they can capture values. The `move` in the second closure in `main` moves the
captured `5` into the closure, as Chapter 20.4 described.

The tests check that the order matters, that an empty pipeline returns its input, and that boxed steps run in order. In `main`, the annotation on the vector is needed, because the two closures have
different types, and only the trait object gives them a common one.

### The extensions

A generic `run_any<F: Fn(i32) -> i32>(steps: &[F], start: i32)` works for one kind of step at a time. Each slice has a single element type `F`, so a slice can hold either function pointers or boxed closures, but
not both. Mixing them would need `Box<dyn Fn>` for every step.

A pipeline that returns a closure fits `impl Fn(i32) -> i32` if the steps are fixed in the function, or `Box<dyn Fn(i32) -> i32>` if the steps come from a list that the caller builds. The second is the same
choice the book made in Listing 20-34.

---

*This walkthrough accompanies the exercise in Chapter 20 of the course. It's not from the book.*
