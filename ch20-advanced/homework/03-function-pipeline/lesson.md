## Homework: A Function Pipeline

**The problem:** A pipeline is a list of steps, and each step takes a number and returns a number. Run a value through the steps in order. Write two versions: one whose steps are function pointers, and one
whose steps are boxed closures, which can capture values.

Create a project in VS Code with `cargo new function_pipeline`. In `src/main.rs`:

* Write `fn double(x: i32) -> i32` and `fn increment(x: i32) -> i32`.
* Write `fn run_fns(steps: &[fn(i32) -> i32], start: i32) -> i32`. Use `fold` to pass the value through each step in order.
* Write `fn run_boxed(steps: &[Box<dyn Fn(i32) -> i32>], start: i32) -> i32`, which does the same thing for boxed closures.
* In `main`, run `[double, increment]` on 5, and `[increment, double]` on 5. Then build a `Vec<Box<dyn Fn(i32) -> i32>>` with a closure that multiplies by 10, and a closure that adds 5 to a captured
  number, and run it on 2.

Write tests in a `tests` module. Check that the order of the steps matters, and that an empty pipeline returns the starting value.

---

### Hints

* `steps.iter().fold(start, |acc, step| step(acc))` runs the pipeline. The accumulator is the value so far.
* A `&[fn(i32) -> i32]` slice can hold named functions, because each one coerces to the pointer type. Passing `&[double, increment]` works without a cast.
* Annotate the boxed vector, `Vec<Box<dyn Fn(i32) -> i32>>`. Each closure is a different type, and the annotation makes them one type.

---

### Check your work

Your program should print:

```
fn pipeline: 11
reversed: 12
boxed: 25
```

`double` then `increment` gives 5 × 2 + 1 = 11. `increment` then `double` gives (5 + 1) × 2 = 12. The boxed pipeline gives 2 × 10 = 20, then 20 + 5 = 25.

---

### Extensions

* Write `fn run_any<F: Fn(i32) -> i32>(steps: &[F], start: i32) -> i32`. Does it accept a mix of function pointers and boxed closures? Why or why not?
* Make a pipeline function that returns a new closure, which runs the whole pipeline. Which return type fits?

---

*This exercise is not from the book. It practices function pointers and closures from Chapter 20.*
