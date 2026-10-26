## Compound Types

A *compound* type groups several values into one. Rust has two built-in ones: the **tuple** and the
**array**. They look alike at first, and the difference between them is the most important thing in
this step.

---

### Part 1: Run the program

Press **F5**. The Output pane should show:

```
The value of y is: 6.4
first: 500, second: 6.4, third: 1
x is 500, z is 1
first: 1, second: 2
```

---

### Part 2: Tuples

A **tuple** groups values of different types, inside parentheses, separated by commas:

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

This line is part of the sample program in Part 1, and prints nothing on its own. It has three parts: an `i32`, an `f64`, and a `u8`. The type annotation is optional, but
when you write it out, the order and types of the parts must match the values exactly.

There are two ways to get at the parts of a tuple:

* **Destructuring** unpacks the tuple into separate variables, all at once: `let (x, y, z) = tup;`.
  Now `x` is `500`, `y` is `6.4`, and `z` is `1`.
* **Dot notation** reaches one part by its position, counting from zero: `tup.0` is `500`,
  `tup.1` is `6.4`, and `tup.2` is `1`.

A tuple with no values at all, `()`, is called the *unit* type. You'll see it again when we look at
functions.

---

### Part 3: Arrays

An **array** also groups values, but every value must be the same type, and the array always has
the same length. It's written in square brackets:

```rust
let a = [1, 2, 3, 4, 5];
```

This line is also part of the sample program, and prints nothing on its own.

You get at a value by its index, which is its position, counting from zero. `a[0]` is `1`, and
`a[1]` is `2`. The index goes in square brackets, after the name of the array.

Arrays are useful when the number of items won't change, such as the days of the week. If you need a
collection that can grow or shrink, Rust has a different type, called a vector, which you'll meet in
Chapter 8.

You can declare an array's type with its length, like `[i32; 5]`, which means "five `i32` values".
You can also make an array with the same value repeated: `[3; 5]` is `[3, 3, 3, 3, 3]`.

---

### Part 4: Tuples or arrays?

Use a **tuple** when you have a fixed group of values that can be different types, like a name, an
age, and a flag. Use an **array** when you have a fixed number of values that are all the same
type, like five test scores.

---

### What happens with a bad index?

We saw that Rust refuses to compile `a[10]` when the index is fixed in the code. But what if the
index comes from the player, and nobody knows it in advance? Rust checks at runtime instead.

To see it, replace `src/main.rs` with this program. You can do that in the Source pane, and press
**Reset** afterwards to get the original back:

```rust
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entry was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
```

Enter `2`. The program prints the element at that position:

```
Please enter an array index.
2
The value of the element at index 2 is: 3
```

Now run it again, and enter `10`. The array only has five elements, so there is no element at
index 10. The program panics:

```
Please enter an array index.
10
thread 'main' (38048) panicked at src\main.rs:16:19:
index out of bounds: the len is 5 but the index is 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `...\compound_types.exe` (exit code: 101)

— Finished with exit code 101 —
```

The panic is the first two lines of the message. The rest is information you can mostly ignore:

* `(38048)` is the process ID, so it will be different on your computer.
* The `note:` line suggests a setting that prints a *backtrace*, which is a list of the functions
  the program was in when it panicked. It's useful for big programs, and you don't need it yet.
* `error: process didn't exit successfully` is cargo reporting that the program ended with a
  problem. The path will be different on your computer.
* Exit code 101 is the number Rust uses when a program panics. If you see 101, look above it for
  the panic message.

The line numbers may differ. The important part is the message: the index was out of bounds, and
the array's length was 5.

Rust stops here on purpose. In some languages, reading past the end of an array returns whatever
happens to be stored in the memory next to it. That can be a bug that's hard to find, or a security
hole. Rust would rather stop the program with a clear message.

---

### Try it

* Change `a[1]` to `a[10]`. Rust can see the index is outside the array, so it stops with a compile
  error. Read the message.
* Read the bad-index example above, and try entering `5`, which is just past the end. Then try `-1`.
  Which of these does Rust reject before the program runs, and which does it reject while the program
  is running? Think about the type of `index`.
* Add `println!("{}", tup.3);` to the program. What does Rust say about the tuple's length?

---

### Quiz

*The first two questions below are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/). The questions about panics are our own.*

````quiz
Does this program compile?

```rust
fn main() {
    let message = "The temperature today is:";
    let x = [message, 100];
    println!("{} {}", x[0], x[1]);
}
```

○ This program does compile
→ Not quite. An array must hold values of one type. `message` is text, and `100` is a number.
◉ This program does not compile
→ Yes. The two elements have different types, so the array has no single type it could have.
````

````quiz
Does this program compile?

```rust
fn main() {
    let t = ([1; 2], [3; 4]);
    let (a, b) = t;
    println!("{}", a[0] + t.1[0]);
}
```

○ This program does not compile
→ Not quite. Both parts of `t` are arrays of numbers, which is fine, and `a[0] + t.1[0]` is valid.
◉ This program does compile
→ Yes. `[1; 2]` and `[3; 4]` are both arrays of numbers. Each can have a different length, because they are separate values.
---
If it compiles, what does it print?

[ 4 ]
→ Yes. `a[0]` is 1, and `t.1[0]` is 3, so the sum is 4.
````

````quiz
Why does Rust stop with a panic when an index is out of bounds, rather than returning some other value?

○ Because arrays can only hold five values
→ Not quite. The array in this step has five values, but that isn't the reason for the panic.
◉ Because reading past the end of an array would read memory that belongs to something else, so Rust stops instead
→ Yes. Stopping with a clear message is safer than returning whatever happens to be in memory.
○ Because the panic makes the program run faster
→ Not quite. The check costs a little time. Rust accepts that cost for safety.
````

````quiz
The player enters `10`, and the array has five elements. What happens?

○ The program prints `0`
→ Not quite. Rust doesn't return a default value for a missing element.
○ The program prints `10`
→ Not quite. `10` is the index, not an element of the array.
◉ The program panics with a message saying the index is out of bounds
→ Yes. Rust checks the index when the program runs, because the value came from the player.
○ The program doesn't compile
→ Not quite. The index came from input, so Rust can't check it until the program runs.
````

---

*For more detail, see section 3.2 "Data Types" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch03-02-data-types.html).*
