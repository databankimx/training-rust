## Scalar Types

A *scalar* type holds a single value. Rust has four of them: integers, floating-point numbers,
booleans, and characters. You may know these from other languages, so most of this step will feel
familiar. The differences are in the details, and Rust is particular about those details, which
is the reason to read this step carefully.

---

### Part 1: Run the program

Press **F5**. The Output pane should show:

```
age 30, big 9000000000, years 2
price 4.99
17 / 5 = 3, remainder 2
7.0 / 2.0 = 3.5
false Z 😻
checked: None
wrapping: 0
```

Each line uses a different scalar type. Let's go through them.

---

### Part 2: Integers

An *integer* is a whole number, with no decimal point. Rust has twelve integer types, which
come in two families:

| Length | Signed | Unsigned |
|---|---|---|
| 8-bit | `i8` | `u8` |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` | `u32` |
| 64-bit | `i64` | `u64` |
| 128-bit | `i128` | `u128` |
| Architecture | `isize` | `usize` |

A **signed** type can hold negative numbers, and it starts with `i`. An **unsigned** type can only
hold zero and positive numbers, and it starts with `u`. The number in the name is how many bits the
type takes up. More bits means a larger range. `u8` holds 0 to 255, and `i32` holds about
-2 billion to +2 billion.

`isize` and `usize` are the sizes that match your computer's architecture: 64 bits on most modern
machines. You'll use `usize` mostly for counting and indexing, which you'll see later.

In the program, `let age: u8 = 30;` says the type out loud. You don't always have to. If you leave
the type off, as in `let years = 2;`, Rust picks `i32`, which is a good default for most whole
numbers.

Integers can be written in several forms. Underscores are allowed to make big numbers easier to read,
so `9_000_000_000` is the same as `9000000000`. Other forms include `0xff` for hexadecimal, `0o77` for
octal, and `0b1111_0000` for binary.

---

### Part 3: Floating-point numbers

A *floating-point* number has a decimal point. Rust has `f32` and `f64`, and `f64` is the default. In
the program, `let price = 4.99;` is an `f64`.

---

### Part 4: Arithmetic

Rust supports the basic operations you'd expect: `+`, `-`, `*`, `/`, and `%` for remainder. Integer
division rounds towards zero, which is why `17 / 5` gives `3`, not `3.4`. The remainder, `17 % 5`,
is the `2` left over.

For decimal results, at least one number must be a decimal. That's why the program writes `7.0 / 2.0`
and gets `3.5`.

---

### Part 5: Booleans and characters

A **boolean** is `true` or `false`, written as `bool`. It's usually the result of a comparison, and
you'll use booleans a lot in the next chapters.

A **character**, `char`, is written in single quotes: `'Z'`. Strings use double quotes, and that's
the difference you'll meet again in Chapter 8. A `char` is four bytes, and it can hold any Unicode
scalar value. That's why `'😻'` works, and so do accented letters and characters from many other
writing systems.

---

### Part 6: Overflow

This is the part that surprises people. A `u8` holds 0 to 255. What happens if you try to store 256?
It depends on how the program was compiled:

* **In debug builds** (the ones the runner builds, and the ones you'll use while you're learning),
  Rust checks for overflow. If it happens, the program panics: it stops with an error.
* **In release builds** (the optimized ones, built with `cargo build --release`), Rust doesn't check.
  Instead, the value wraps around. 256 becomes 0, and 257 becomes 1.

Relying on the wrap-around is considered an error, even though it happens. If you want to deal with
overflow on purpose, the standard library gives you methods for each choice:

* `checked_add` returns `None` if the result overflows, and the number otherwise.
* `wrapping_add` always wraps around.
* `saturating_add` stops at the maximum value.
* `overflowing_add` returns the result and a flag that says whether it overflowed.

The program uses `checked_add` and `wrapping_add` on `255`. The first gives `None`, and the second
gives `0`. The `{:?}` in `println!` is a way of printing a value in its debug form, so you can see
the `None`.

---

### Try it

* Change `let age: u8 = 30;` to `let age: u8 = 300;`. What does Rust say? Is it a compile error, or
  a panic at runtime? (Hint: Rust can sometimes see the problem before the program runs.)
* Add these lines to the end of `main`, and run it:

  ```rust
  let almost_max: u8 = 255;
  let overflowed = almost_max + 1;
  println!("{overflowed}");
  ```

  The message is similar to this:

  ```text
  error: this arithmetic operation will overflow
  ```

  Rust can see that this addition overflows, so it usually stops with a compile error rather than
  a panic. Read the message. Then delete the lines, so the program works again.
* Change `17 / 5` to `17.0 / 5.0`, and check the result. Why does the decimal version work without
  a change to the type?

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
The largest number representable by the type `i128` is:

○ 2^128
→ Not quite. That would need 129 bits to store. `i128` uses 128 bits, and one of them is for the sign.
○ 2^128 - 1
→ Not quite. That's the largest value of an unsigned 128-bit integer, `u128`.
○ 2^127
→ Not quite. That's one more than the largest signed value, which can't be stored.
◉ 2^127 - 1
→ Yes. A signed integer uses one bit for the sign, so the largest value is one less than 2^127.
````

````quiz
If `x` is a `u8` and its value is 0, what happens when the program computes `x - 1`?

○ It will always panic
→ Not quite. That's true in debug builds, but not in release builds.
○ It will always return 255
→ Not quite. In release builds it wraps to 255, but in debug builds it panics.
◉ It depends on the compiler mode
→ Yes. Debug builds panic on overflow, and release builds wrap around.
````

````quiz
Does this program compile?

```rust
fn main() {
    let x: fsize = 2.0;
    println!("{x}");
}
```

○ This program does compile
→ Not quite. `fsize` is not a type in Rust.
◉ This program does not compile
→ Yes. The types for floating-point numbers are `f32` and `f64`.
````

---

*For more detail, see section 3.2 "Data Types" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch03-02-data-types.html).*
