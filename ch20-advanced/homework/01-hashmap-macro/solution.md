## Solution: A hashmap! Macro

This walkthrough explains the example solution in `src/main.rs`. Read it after you've written your own version.

### Run the solution

Press **Run** for the output, and **Test** for the three tests. The output is:

```
ann: 31
ben: 25
```

### How it works

The pattern `$( $key:expr => $value:expr ),*` matches zero or more pairs, and each `$key` and `$value` is a complete expression. Because `$key:expr` is followed by `=>`, the compiler accepts that
separator. The optional `$(,)?` accepts one trailing comma, and matches nothing when there isn't one.

The body uses two pairs of braces. The outer pair makes the expansion a single expression, so it can be used in a `let`. The inner block holds the statements: create the map, insert each pair, and return it.
The map's type is inferred from the inserts, so the macro needs no type written out.

The macro uses the full path `std::collections::HashMap`. A path written in the macro resolves at the place where the macro is called, so a caller doesn't need to import anything. The `#[allow(unused_mut)]` attribute stops the
warning for an empty call, where `mut` isn't needed.

`main` sorts the keys, because a `HashMap` has no order, and then looks up each value. The tests check the number of entries after a trailing comma, the empty case, and a lookup.

### The extensions

Accepting `()` or `[]` as well as `{}` doesn't change the macro. Braces, parentheses, and brackets are all delimiters that the caller chooses when invoking the macro, so the pattern is the same for each. The only thing
that would change is the documentation of the macro's use.

A second arm that takes plain values as both key and value would be tried after the first arm, if the first arm doesn't match. Macro arms are checked in order, so the more specific arm should come first.

---

*This walkthrough accompanies the exercise in Chapter 20 of the course. It's not from the book.*
