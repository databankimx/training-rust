## Constants and Shadowing

In the last step, you learned that variables are immutable by default. Rust has two more tools
that look similar at first, and this step is about telling them apart: **constants** and
**shadowing**.

By the end of this step, you'll know when to use each one, and why the two are not the same thing,
even though they both give a name to a value.

---

### Part 1: Run the program

Press **F5** with the code in this step. Your Output pane should show:

```
The value of x in the inner scope is: 12
The value of x is: 6
Seconds in three hours: 10800
Number of spaces: 3
```

Each line comes from a different part of the program, and you'll read each part in turn.

---

### Part 2: Read the code

Here is the program with comments on the new ideas:

```rust
// A constant: its value is fixed, and it must be written in UPPER_SNAKE_CASE.
// The type (u32) is required, and the value must be known when the program is compiled.
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x = 5;

    // Shadowing: this second `let x` creates a new variable that hides the first one.
    let x = x + 1;

    {
        // This `x` only exists inside these braces. It hides the outer `x` in here.
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    // Outside the braces, the inner `x` is gone, so this prints the value from before.
    println!("The value of x is: {x}");
    println!("Seconds in three hours: {THREE_HOURS_IN_SECONDS}");

    // Shadowing can also change the type. This `spaces` is text first, then a number.
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Number of spaces: {spaces}");
}
```

The output is the same as in Part 1.

---

### Constants

A constant is declared with `const`, not `let`. The differences are important:

* A constant must have a type. Here it's `u32`, which the compiler needs to know ahead of time.
* A constant can be declared in the global scope, outside any function. A `let` can't.
* A constant's value must be something the compiler can work out before the program runs, such as
  `60 * 60 * 3`. It can't come from user input or from a function call that runs at runtime.
* Constants are always immutable. `mut` isn't allowed with `const`, because the value never changes.
* By convention, constant names use `UPPER_SNAKE_CASE`. Rust warns you if they don't, but it
  won't stop the program.

---

### Shadowing

Shadowing is different from `mut`. When you write `let x = ...` a second time, you're not changing
the first `x`. You're creating a new variable that happens to have the same name, and the old one
is hidden.

Two things follow from that:

* You can change the type of a value through shadowing, as `spaces` does. A `mut` variable can't
  do that, because its type is fixed when it's declared.
* A shadowed variable goes out of scope at the closing brace of the block it was declared in. The
  inner `x` in the example disappears at the `}`, and the outer `x` comes back into use.

Shadowing is most useful when you want to take a value, transform it, and then carry on using the
result under the same name, without needing a new name for each step.

---

### Try it

* Change `THREE_HOURS_IN_SECONDS` to `const ONE_DAY: u32 = 60 * 60 * 24;`, and print that instead.
  What do you see?
* Try writing `const threeHours: u32 = 10800;`. Does Rust complain? Is it an error or a warning?
* Change `let spaces = spaces.len();` to `spaces = spaces.len();`, and add `mut` to the first
  line, so it reads `let mut spaces = "   ";`. What does Rust tell you about the type?

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Which statement is correct about the difference between `let` and `const`?

○ A `const` can only be assigned a literal, not an expression that involves calculation
→ Not quite. The example in this step uses `60 * 60 * 3`, which is an expression. Constants can use expressions that the compiler can work out ahead of time.
○ They are just different syntaxes for declaring variables with the same meaning
→ Not quite. Constants can be declared globally, must have a type, and can never be `mut`. A `let` variable does none of these.
○ The compiler gives an error if a `const` name isn't in UPPER_SNAKE_CASE
→ Not quite. That's a convention. Rust warns you about it, but it doesn't stop the program.
◉ A `const` can be declared in the global scope, and a `let` can only be used inside a function
→ Yes. Constants live at the top level of the file, while variables usually live inside functions.
````

````quiz
Does this program compile?

```rust
const TWO: u32 = 1 + 1;

fn main() {
    println!("{TWO}");
}
```

○ This program does not compile
→ Not quite. `1 + 1` can be worked out before the program runs, so the constant is allowed.
◉ This program does compile
→ Yes. The compiler can work out `1 + 1` before the program runs, so it's a valid constant.
---
If it compiles, what does it print?

[ 2 ]
→ Yes. The program prints the value of `TWO`, which is `1 + 1`.
````

````quiz
The program prints two lines. What does the second one, printed outside the braces, say?

```rust
fn main() {
    let x = 1;
    {
        let x = x + 1;
        let x = x * 2;
        println!("{x}");
    }
    println!("{x}");
}
```

[ 1 ]
→ Yes. The first line prints 4, from the inner `x`. That `x` disappears at the closing brace, so the second line prints the outer `x`, which is still 1.
````

````quiz
Does this program compile?

```rust
fn main() {
    let mut x: u32 = 1;
    x = "Hello world";
    println!("{x}");
}
```

○ This program does compile
→ Not quite. `x` has the type `u32`, a number, so it can't hold text.
◉ This program does not compile
→ Yes. Shadowing could change the type, but `mut` can't. Writing `let x = "Hello world";` instead would compile.
````

---

*For more detail, see section 3.1 "Variables and Mutability" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html).*
