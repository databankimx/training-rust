## The Option Enum

Many languages have a special value, `null`, that means there's no value. Rust doesn't have it. The problem with `null` is that a
variable of a type that can be null looks the same as one that can't, so code can use a null value as if it were valid, and fail at runtime.

Rust solves this with an enum from the standard library, `Option<T>`. It says that a value is either present or absent, and the type
system makes you deal with both cases:

```rust,ignore
enum Option<T> {
    None,
    Some(T),
}
```

`Some(T)` holds a value of any type, and `None` holds nothing. The `<T>` is a generic type parameter, which Chapter 10 covers in detail. For
now, it means that `Some` holds one value, of whatever type you use.

`Option` is in the prelude, which means `Some` and `None` are available without any `use` line, and without the `Option::` prefix.

---

### Some and None in practice

The sample holds a number, a character, and an absent value:

```rust,ignore
let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;
```

The sample's output is:

```
Some(5) Some('e') None
```

The types differ. `some_number` is `Option<i32>`, and `some_char` is `Option<char>`, which is a different type. Rust infers them from the values
inside `Some`. `None` holds no value, so Rust can't infer the type inside it, and the annotation `Option<i32>` is needed.

---

### Why Option is safer than null

`Option<T>` and `T` are different types. The compiler won't let you use an `Option<T>` as if it were a `T`. This code tries to add an `i8` to an
`Option<i8>`, and it doesn't compile:

```rust,ignore
let x: i8 = 5;
let y: Option<i8> = Some(5);
let sum = x + y;
```

The error says that Rust can't add an `Option<i8>` to an `i8`, because they are different types. A value of type `i8` is always a number, so you
can use it without checking. An `Option<i8>` might be empty, so you have to handle that case before you can use the number inside.

This is the design's main point. If a value can be absent, its type says so. If its type isn't an `Option`, the compiler guarantees it's present.
You have to opt in to possible absence, and then you must handle it.

To get the value out of an `Option`, you handle each variant. The `match` expression does this, and Chapter 6.2 covers it in detail.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
Some(5) Some('e') None
```

The comments in `src/main.rs` suggest changing `Some(5)` to `Some(5.0)`. The type of `some_number` becomes `Option<f64>`, which is a different type from
`Option<i32>`, but the program still compiles.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Does this program compile?

```rust
fn foo(x: &i32) {
  println!("{x}");
}

fn main() {
  let x = null;
  foo(x);
}
```

○ This program does not compile
→ Yes. Rust has no `null` keyword, so the program can't use a null value. An `Option` would be used to represent a value that may be absent.
○ This program does compile
→ Not quite. `null` isn't a keyword in Rust, so `let x = null;` is rejected.
---
On which line does Rust report the problem?

[ 6 ]
→ Yes. Line 6 uses `null`, which doesn't exist in Rust.
````

````quiz
Consider these two representations of a `Result` type that contains a value `T` if a computation succeeds, or an error `E` if it fails.

```
struct Result1<T, E> {
  ok: Option<T>,
  err: Option<E>,
}

enum Result2<T, E> {
  Ok(T),
  Err(E)
}
```

The enum `Result2` is considered more idiomatic than the struct `Result1` in Rust. Which statement below is **NOT** a valid reason why?

○ The struct uses more space in memory at runtime than the enum
→ This is a valid reason, so it isn't the answer. The struct has two fields, and each `Option` takes space even when it's `None`.
○ The struct could have `ok` and `err` both be `None`, while the enum must have at least one of them
→ This is a valid reason, so it isn't the answer. With the struct, nothing stops both fields being `None`, which the enum rules out.
○ The struct is more syntactically verbose to construct than the enum
→ This is a valid reason, so it isn't the answer. The struct needs field names and `Option` wrappers, while the enum variants are short to write.
◉ The struct contains `Option` types, which are only intended to wrap structs
→ Yes. This is the answer, because `Option` can wrap any type, and a struct field of type `Option` is allowed. Using `Option` fields isn't the problem.
````

---

*For more detail, see section 6.1 "Defining an Enum" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html).*
