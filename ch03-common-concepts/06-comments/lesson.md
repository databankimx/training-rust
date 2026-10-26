## Comments

Every program you write will be read by someone, and that someone is often you, a few months
later. A *comment* is a note in the code that's for people. Rust ignores it completely when it
compiles the program.

In this step, you'll learn how to write comments, and more importantly, what they're for.

---

### Part 1: Run the program

Press **F5**. The Output pane should show:

```
Hello, world!
```

Nothing else. The comments don't print, and the line that's commented out doesn't run either.

---

### Part 2: Write a comment

A line comment starts with two slashes, `//`. Everything from the slashes to the end of the line is
ignored:

```rust
// This line is a comment. Rust ignores it when it compiles the program.
```

A comment can also go at the end of a line of code:

```rust
println!("Hello, world!"); // Comments can also go at the end of a line.
```

The `println!` runs, and the comment after the `//` is ignored.

---

### Part 3: Comment out code

A comment can also hide a line of code that you don't want to run, without deleting it. This is
useful when you're experimenting:

```rust
// println!("This line is commented out, so it won't run.");
```

Remove the `//` and the line runs again. This is one of the most common uses for comments while
you're learning, and experienced programmers use it too.

---

### Part 4: What makes a good comment

Write comments to explain *why* the code does something, or what it's for, when that isn't obvious
from the code itself. Don't repeat what the code says:

```rust
// Print a greeting. (This is a poor comment: the code already says this.)
println!("Hello, world!");
```

```rust
// The output is buffered, so we flush it before asking for input. (This explains why.)
```

These are examples of comments, not complete programs.

Good comments are short. If a comment needs several paragraphs, the code may need to be simpler.

---

### Try it

* Add a comment above `println!` that says what the program does.
* Comment out the `println!` line, and run the program. What appears in the Output pane?
* Write a comment that explains *why* the program prints a greeting, rather than *what* it prints.

---

### Quiz

````quiz
What do line comments in Rust start with?

[ // ]
→ Two slashes. Everything after them, up to the end of the line, is ignored by the compiler.
````

````quiz
Which of these is the best reason to write a comment?

○ To repeat what the code already says, so that readers don't have to work it out
→ Not quite. The code already says what it does, so repeating it adds nothing.
◉ To explain why the code does something, when that isn't clear from the code alone
→ Yes. Comments are most useful when they explain intent or a reason.
○ To make the program run faster
→ Not quite. Comments are ignored by the compiler, so they have no effect on speed.
````

---

*For more detail, see section 3.4 "Comments" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch03-04-comments.html).*
