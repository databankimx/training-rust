## Homework: A hashmap! Macro

**The problem:** Writing a `HashMap` takes several lines of `insert` calls. Write a declarative macro, `hashmap!`, that builds one from `key => value` pairs, like this: `hashmap!{ "ann" => 31, "ben" => 25 }`. The
macro should also accept a trailing comma, and an empty list.

Create a project in VS Code with `cargo new hashmap_macro`. In `src/main.rs`:

* Write `macro_rules! hashmap` with one arm. The pattern should be `$( $key:expr => $value:expr ),* $(,)?`. The `$(,)?` allows an optional trailing comma.
* In the body, create a `HashMap` with `HashMap::new()`, insert each pair, and return the map. The body needs an extra pair of braces, so the macro returns one expression.
* In `main`, build a map from two pairs, and print the entries in sorted order by key.

Write tests in a `tests` module. Check that a macro call with three pairs and a trailing comma has three entries, that an empty call gives an empty map, and that one lookup returns the right value.

---

### Hints

* The expansion needs `use std::collections::HashMap;`, or the full path `std::collections::HashMap::new()` in the macro. A full path works wherever the macro is used.
* Collect the keys into a `Vec`, and sort it, before printing. `HashMap` doesn't keep an order.
* An empty map has a type the compiler can't always infer. Annotate it in the `let`, as `let empty: HashMap<&str, i32> = hashmap!{};`.

---

### Check your work

Your program should print:

```
ann: 31
ben: 25
```

---

### Extensions

* Allow the macro to be called with `()` or `[]` as well as `{}`. Which part of the macro needs to change?
* Add a second arm that takes a list of plain values and uses the value as the key too. Does the order of the arms matter?

---

*This exercise is not from the book. It practices declarative macros from Chapter 20.*
