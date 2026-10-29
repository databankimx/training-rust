## Deriving Debug

Printing a struct is useful when you're debugging, because it shows the values of all its fields. But `println!` can't
print a struct by default, and the reason is worth understanding.

---

### Why a struct won't print

The `{}` placeholder asks for the `Display` format, which is for presenting a value to an end user. Numbers and strings have an obvious
`Display` form. A struct doesn't, because there are many reasonable ways to print one: with or without commas, with or without its
field names, and with or without all the fields. So Rust doesn't guess, and a struct has no `Display` implementation unless you write one.

If you try to print a `Rectangle` with `{}`, the compiler reports that the type doesn't implement `Display`.

---

### The Debug format

The `Debug` format is for developers. It's written with `{:?}`, and it shows the value in a form that's useful while debugging.
Rust doesn't provide `Debug` for your struct automatically either, so you opt in with an attribute placed above the definition:

```rust,ignore
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

`#[derive(Debug)]` asks the compiler to write the `Debug` implementation for you. The sample prints the struct in both styles:

```rust,ignore
println!("rect1 is {rect1:?}");
println!("rect1 is {rect1:#?}");
```

The first form prints on one line, and the second spreads the fields over several lines, which is easier to read for large structs.

---

### The sample

The sample is the rectangle program with a `dbg!` call added. Here it is in full:

```rust,ignore
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        // dbg! prints the value to standard error, and then returns it, so width is still 60.
        width: dbg!(30 * scale),
        height: 50,
    };

    // dbg! takes ownership of its argument, so pass a reference to keep rect1 for later use.
    dbg!(&rect1);

    // {:?} uses the Debug format on one line, and {:#?} spreads it over several lines.
    println!("rect1 is {rect1:?}");
    println!("rect1 is {rect1:#?}");

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

The `width` is `30 * scale`, which is `60`, so the rectangle is 60 pixels wide and 50 pixels high. Its output is:

```
[src\main.rs:11:16] 30 * scale = rect1 is Rectangle { width: 60, height: 50 }
rect1 is Rectangle {
    width: 60,
    height: 50,
}
The area of the rectangle is 3000 square pixels.
60
[src\main.rs:16:5] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```

The output looks out of order, and the reason is worth understanding. The `dbg!` calls write to standard error, and `println!`
writes to standard output. The two streams are separate, so the runner can show their text in a different order from the one
in which the code ran. Here, the first `dbg!` line starts before the `rect1` lines, and its value, `60`, appears after the area line.
The order you see can vary a little, but the text of each line is always the same.

---

### The dbg! macro

`dbg!` is another way to print a value in the `Debug` format. Unlike `println!`, it takes ownership of the expression it's given, and it
returns that value, so it can wrap an expression without changing the program's behavior. It also prints the file name, line, and column
where it was called.

In the sample, `dbg!(30 * scale)` prints the expression and its value, and then returns `60`, so `width` gets the same value as before.
`dbg!(&rect1)` prints the struct in the pretty `Debug` format. Because `dbg!` takes ownership, the sample passes a reference, so `rect1`
can still be used afterwards.

`dbg!` writes to standard error, which is the stream for diagnostic output, rather than standard output, which is for a program's results.

---

### Try it in the runner

Press **F5**. The Output pane should show the output above, including the lines from standard error.

The comments in `src/main.rs` suggest removing `#[derive(Debug)]`. Rust rejects the `{:?}` lines, and the note explains that the struct
does not implement `Debug`.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Which statement best describes a difference between the `Display` and `Debug` traits?

○ `Display` is for printing values to the console, while `Debug` is for viewing values in a debugger
→ Not quite. Both print to the console. The difference is who the output is for, not where it goes.
○ `Display` cannot be implemented for structs, while `Debug` can be implemented for structs
→ Not quite. Both traits can be implemented for structs. `Debug` is simply the one Rust can provide automatically.
○ There is no difference, `Display` and `Debug` are aliases for the same trait
→ Not quite. They are separate traits, with separate formats.
◉ `Display` is for presenting values to an end-user, while `Debug` is for developers' internal use
→ Yes. `Display` gives a clean form for people to read, and `Debug` gives a form that helps you see what the value holds.
````

---

*For more detail, see section 5.2 "An Example Program Using Structs" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch05-02-example-structs.html).*
