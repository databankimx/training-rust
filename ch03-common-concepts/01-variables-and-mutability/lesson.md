## Variables and Mutability

A *variable* is a name that holds a value. You'll use them in almost every line of every
program, so this is the first place where Rust's rules show up. The rules are strict, and
they're there for a good reason: they stop you changing something by accident.

In this step, you'll see how Rust handles variables, and what happens when you try to change one
that wasn't meant to change.

---

### Part 1: Make the program fail

Create a new Cargo project in your Rust Training folder, or open the one in this step. Then edit
`src/main.rs` so it reads:

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

Press **F5**. The program won't compile. Rust reports an error that says it can't assign twice
to the variable `x`. The compiler stops with an error like this:

```text
error[E0384]: cannot assign twice to immutable variable `x`
```

The exact wording may differ slightly between versions, but the message
is about `x` being immutable.

Read the error from top to bottom. Rust usually points at the line that caused the problem, with
a small arrow under the part of the line it means. In this case, the arrow points at `x = 6`.

---

### Part 2: Make the variable mutable

Now add one word, `mut`, to the first line. The program should read:

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

Press **F5**. This time the program compiles and runs. The Output pane shows:

```
The value of x is: 5
The value of x is: 6
```

The only change was the `mut`. That tells Rust that `x` is allowed to change, so the second
assignment is allowed too.

---

### Part 3: Read the code

Here is the working program with comments on the new ideas:

```rust
fn main() {
    // let creates a variable called x and gives it the value 5.
    // mut means the value can be changed later. Without it, x is immutable.
    let mut x = 5;
    println!("The value of x is: {x}");

    // This assignment only works because x was declared with mut.
    x = 6;
    println!("The value of x is: {x}");
}
```

The output is the same as in Part 2.

Variables in Rust are **immutable by default**. Once a value is given to a variable, it can't
change unless you ask for that with `mut`. This feels strange if you've used other languages, where
variables can usually be changed freely. Rust makes you say so explicitly, because a variable that
changes unexpectedly is one of the most common sources of bugs.

Read the code again, and notice how little you had to write to make `x` mutable: one extra word.
It's not a complicated rule. It's just a deliberate one.

---

### Try it

* Remove `mut` again, and read the error one more time. Can you see the arrow pointing at `x = 6`?
* Change the second value from `6` to `10`, and check the output.
* Add a third line that assigns `x = 7;` and prints `x`. Does it still compile?
* What happens if you write `let mut x = 5;` and then never change `x`? Try it. Rust gives you a
  warning, not an error. Why would it bother to warn you?

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Which statement best describes what it means if a variable `x` is immutable?

○ `x` is stored in the immutable region of memory.
→ Not quite. Rust doesn't split memory into regions for this. Immutability is a rule about the variable itself.
○ After being defined, `x` can be changed at most once.
→ Not quite. An immutable variable can't be changed at all after it's given a value.
○ You cannot create a reference to `x`.
→ Not quite. You can create references to immutable variables. It's changing the value that's forbidden.
◉ `x` cannot be changed after being assigned a value.
→ Yes. That's what immutable means, and it's the default in Rust.
````

````quiz
What is the keyword you put after `let` to indicate that a variable can be changed?

[ mut ]
→ `mut` is short for mutable. It goes between `let` and the variable's name, such as `let mut x = 5;`.
````

````quiz
Does this program compile?

```rust
fn main() {
    let x = 1;
    println!("{x}");
    x += 1;
    println!("{x}");
}
```

○ This program does compile
→ Not quite. `x += 1` changes `x`, and `x` was not declared with `mut`.
◉ This program does not compile
→ Yes. Add `mut` to the `let` line, and it will compile and print `1` and then `2`.
````

---

*For more detail, see section 3.1 "Variables and Mutability" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html).*
