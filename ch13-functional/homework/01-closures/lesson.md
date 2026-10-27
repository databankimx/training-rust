## Homework: Closures

**The problem:** Practice writing functions that take and return closures.

Create a project in VS Code with `cargo new closures_homework`. In `src/main.rs`, write:

* `make_counter() -> impl FnMut() -> u32`, which returns a closure. Each call to the closure returns the next number, starting at 1: 1, 2, 3, and so on. The closure
  owns its own count, so `make_counter` doesn't need any shared state.
* `apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32`, which applies `f` to `x`, and then applies `f` to the result.
* `sort_by_length(words: &mut Vec<&str>)`, which sorts the words from longest to shortest, using `sort_by_key` with a closure. Words of equal length can stay in any order.

Then write tests for each function, in a `tests` module, and a `main` that prints a few results.

---

### Hints

* Use `move` in `make_counter`, so the closure takes ownership of its counter.
* `apply_twice(|x| x * 3, 2)` applies "triple" twice, so it should return 18.
* To sort from longest to shortest, the key can be `std::cmp::Reverse(word.len())`.

---

### Check your work

Your `main` should print these lines:

```
counter: 1 2 3
apply_twice: 18
sorted: ["quick", "brown", "jumps", "fox"]
```

The `main` sorts the words `["fox", "quick", "brown", "jumps"]`. `sort_by_key` is a stable sort, so words of equal length keep the order they had. The three
five-letter words therefore stay in their original order, and `fox` comes last. Your tests can check the lengths, if you want to allow for a different sort.

---

### Extensions

* Make `apply_twice` accept any number of applications, given as an argument. Which trait does the closure need, and why?
* Write a `make_adder(n: i32) -> impl Fn(i32) -> i32`. Does it need `move`, and what happens if you leave it out?

---

*This exercise is not from the book. It practices closures from Chapter 13.*
