## Guess the Number: Reading Input

Chapter 1 was about getting Rust running. This chapter is about building something, and
the something is a number guessing game. The computer thinks of a number. You guess it.
The computer tells you whether you're too high, too low, or right. Simple enough to finish
in an afternoon, and rich enough to teach you most of the Rust you'll use every day.

In this first step, we'll do the least exciting part: asking the player for a guess and
reading back what they typed. Think of it as a warm-up. You'll be surprised how much
there is in one line of input, and how much fuss Rust makes about it. The fuss is the
point.

---

### Part 1: Look at the project

This step is a Cargo project called `guessing_game`. Its `Cargo.toml` looks like this:

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"

[dependencies]
```

This is the project file, so it has no output of its own. You met `Cargo.toml` in the last chapter. The `[dependencies]` section is empty for now,
because this step doesn't need any libraries. You'll add one in the next step.

The program is in `src/main.rs`. Open it, and you'll see this:

```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // By default, Rust variables are immutable
    // The 'mut' keyword declares the variable as mutable
    let mut guess = String::new();

    // read_line() returns a Result (enum: Ok | Err)
    //             .expect() executes if Result is Err
    //                       and returns the value if Ok
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

The output of this program is shown in Part 2.

Don't worry if it looks like a lot at once. We'll go through it one piece at a time.

---

### Part 2: Run it

Press **F5**. The Output pane should show:

```
Guess the number!
Please input your guess.
```

Then the program stops and waits. The cursor is in the input box at the bottom of the Output
pane, so type a number, such as `6`, and press **Enter**. The output continues:

```
6
You guessed: 6

```

Your guess appears in green, so you can tell it apart from what the program prints. Then
the program finishes.

Try a different number, or a word, or nothing at all. Press **Enter** with an empty box.
Every one of them works, and the program simply echoes back what you typed. That's
suspicious, and it's also the reason the next steps exist.

---

### Part 3: Read the program

Here is the program again, one piece at a time.

**`use std::io;`** brings the input and output tools into scope. Rust keeps most of its
useful tools in a *standard library*, and `std::io` is the part for input and output. The
`use` line means "I'd like to refer to these by their short names".

**`fn main() {`** starts the program, as you learned in the last chapter.

**`println!("Guess the number!");`** prints a line. You've seen `println!` before, so this
should feel familiar. You can tell it's a macro by the `!`.

**`let mut guess = String::new();`** is the line that needs a closer look. It does three
things at once:

* `let` creates a *variable*, which is a name for a value. Variables are immutable by
  default, which means you can't change them once they have a value. That sounds annoying,
  and it's deliberate. It stops you changing something by accident.
* `mut` makes the variable *mutable*, which means it can be changed. We need that here,
  because we're going to fill `guess` with whatever the player types.
* `String::new()` makes a new, empty `String`. A `String` is a piece of text that can grow
  and shrink. The `::` means "the `new` function that belongs to `String`". You'll meet that
  pattern a lot.

**`io::stdin()`** gives us the standard input, which is the place where typed text comes
from. Notice that we wrote `io::stdin` and not `std::io::stdin`. That's because of the `use`
line at the top.

**`.read_line(&mut guess)`** reads one line of text and puts it into `guess`. The `&mut`
part needs a little explanation:

* `&` means "a reference to". It lets `read_line` look at `guess` without taking ownership of
  it.
* `mut` means "and I allow you to change it". `read_line` needs to add the text into `guess`,
  so it needs permission to change it.

Rust makes you say both of these, on purpose, so you always know where a value can be
changed.

**`.expect("Failed to read line");`** deals with the possibility that reading the input
fails. `read_line` returns a result that might be an error. `expect` says: "If it failed,
stop the program and show this message." Later chapters will show you better ways to handle
errors. This one is fine for now.

**`println!("You guessed: {guess}");`** prints the text. The `{guess}` inside the string
is replaced with the value of `guess`, and that's why your number appears in the output.

---

### Why there's an extra blank line

Look at the output again:

```
You guessed: 6

```

That extra line is real. When you press Enter, the newline character is part of what
`read_line` reads, and it's stored in `guess` along with your number. So `guess` contains
`6` followed by a newline, and `println!` adds another one. Rust doesn't tidy that up for
you.

You'll see the fix later in the chapter, when we compare the guess with the secret number.
For now, notice that it's there, and move on.

---

### Try it

* Change the first two messages to something more inviting. Run the program again.
* Add a line that prints `guess` again, after the first print. What do you see?
* Remove `.expect("Failed to read line");` and replace it with a semicolon. Rust will refuse
  to compile the program. Read the error, then put the line back.

---

### Quiz

````quiz
What does `let mut guess = String::new();` create?

○ A number that can't be changed
→ Not quite. `String` is a piece of text, so this line doesn't create a number. A number would need a type such as `u32`.
○ A variable that can't be changed, and holds the text "guess"
→ Not quite. `guess` is the name of the variable, not its value, and `mut` means the value can be changed.
◉ A new, empty `String` called `guess`, which can be changed
→ Yes. `let` makes the variable, `String::new()` gives it an empty value, and `mut` allows it to change.
○ A reference to a value stored somewhere else
→ Not quite. That's what the `&` in `&mut guess` means. Here, `guess` holds the text itself.
````

````quiz
Why does the program write `&mut guess` when it calls `read_line`?

○ Because `read_line` only works on numbers
→ Not quite. `read_line` reads text, and `guess` is a `String`.
◉ Because `read_line` needs permission to change `guess`, so the typed text can be stored in it
→ Yes. The `&` means "a reference to" and the `mut` means "you may change what it refers to".
○ Because the `&` makes the program run faster
→ Not quite. The `&` is about permission and ownership, not speed.
○ Because `guess` must be printed later
→ Not quite. Printing doesn't need `&mut`. Changing the value is the reason.
````

---

*For more detail, see the section "Processing a Guess" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html).*
