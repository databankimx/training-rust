## Enums for Multiple Types, and Dropping

A vector holds values of one type, which can be limiting. A spreadsheet row, for example, may contain whole numbers, decimals, and text. An enum
solves this: its variants can hold different types, and all of them are one type, the enum. A vector of the enum can then hold a mix.

---

### The spreadsheet example

The sample defines an enum with a variant for each kind of cell:

```rust,ignore
#[allow(dead_code)]
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
```

The `#[allow(dead_code)]` line is needed here. Without it, the compiler warns that the fields are never read, because `Debug` printing
doesn't count as reading each field. A `match` on the cells would read them, as Chapter 6.2 showed.

A vector of `SpreadsheetCell` holds a whole row:

```rust,ignore
let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

Printing the row with `{cell:?}` shows each variant and its value. The output is:

```
Int(3)
Text("blue")
Float(10.12)
```

Rust needs the exact set of types at compile time, so it knows how much space each element takes. An enum makes that set explicit. When you
match on the cells, the compiler checks that every kind is handled, as you saw in Chapter 6.

If you don't know every type a program will need at runtime, an enum won't do. Chapter 18 covers trait objects, which handle that case.

---

### Dropping a vector

A vector is freed when it goes out of scope, like any other struct. When it's freed, all of its elements are dropped too. The borrow checker makes
sure that any reference to an element is used only while the vector is still valid, so no reference outlives the values it refers to.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Int(3)
Text("blue")
Float(10.12)
```

The comments in `src/main.rs` suggest adding `SpreadsheetCell::Int("five")`. Rust rejects it, because the `Int` variant holds an `i32`, not a string.

---

*For more detail, see section 8.1 "Storing Lists of Values with Vectors" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch08-01-vectors.html).*
