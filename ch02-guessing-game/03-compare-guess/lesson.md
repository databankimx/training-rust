## Compare the Guess with the Secret

The game can now make a secret number and read a guess, but the two never meet. In this step,
we'll compare them and tell the player whether they were too small, too big, or right.

We'll also remove the line that prints the secret. A guessing game that shows the answer
before you guess isn't much of a game.

---

### Part 1: Run it

Press **F5**. The Output pane should show:

```
Guess the number!
Please input your guess.
```

Type a number such as `50` and press **Enter**. Your output will be one of these, depending on
the secret number:

```
You guessed: 50
Too small!
```

```
You guessed: 50
Too big!
```

```
You guessed: 50
You win!
```

Run it a few times. You won't see the secret number, so you'll have to work it out from the
hints, which is exactly the point.

---

### Part 2: Read the new code

The code is in `src/main.rs`. Here it is with comments on the new ideas. The comments are for
this lesson only. The file in your project has none.

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // trim() removes the newline from the end of the input.
    // parse() turns the text into a number. The `: u32` tells Rust which kind of number to make.
    // expect() stops the program with this message if the text is not a number.
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    // cmp() compares two numbers and gives back an Ordering: Less, Greater, or Equal.
    // match runs the arm that matches the Ordering it was given.
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

The output of this program is shown in Part 1.

Here's what's new.

**`use std::cmp::Ordering;`** brings in the `Ordering` type, which is an *enum*. An enum is a
type with a fixed list of possible values. `Ordering` has exactly three: `Less`, `Greater`, and
`Equal`. We'll look at enums properly in a later chapter.

**`let guess: u32 = guess.trim().parse().expect(...)`** does two things. First, the new
`guess` replaces the old one, which is allowed in Rust. This is called *shadowing*, and it lets
us reuse a good name for a value with a different type. The old `guess` was text, and the new
one is a number. Second, `trim()` removes the newline that `read_line` left on the end, and
`parse()` turns the text into a number. We need to say which kind of number, so we write
`: u32`, which means an unsigned 32-bit whole number. It's large enough for this game.

**`guess.cmp(&secret_number)`** compares the two numbers. The `&` lets `cmp` look at
`secret_number` without taking it. The result is one of the three `Ordering` values.

**`match`** chooses what to do based on a value. Each line is an *arm*: a pattern, then `=>`,
then the code to run. Rust checks that you've covered every possible value, so none can be
forgotten. Try removing one of the arms, and Rust will refuse to compile the program.

---

### Part 3: Try the errors

Rust's error messages are one of its best features, so it's worth seeing a few.

1. In `src/main.rs`, remove `: u32` from the `guess` line, so it reads
   `let guess = guess.trim().parse().expect(...)`. Press **F5**. Rust can't tell what kind of
   number you want, and it says so.
2. Put `: u32` back. Then delete the `Ordering::Equal` arm. Press **F5**. Rust tells you a case
   is missing.
3. Put everything back the way it was.

---

### Part 4: What happens with text?

Type `hello` as your guess and press **Enter**. The program stops with an error that includes
`Please type a number!`. The output looks like this, though the line numbers depend on your version:

```
thread 'main' panicked at src\main.rs:16:...
Please type a number!
```

That's `expect` doing its job, but it's not a friendly way to end.
In the next step, we'll let the program keep going, so the player can try again.

---

### Try it

* Change the secret range to `1..=10`. Does the game feel easier?
* Print a message for `Ordering::Equal` that's more enthusiastic.
* Type `100` and then `1` in separate runs. What do you notice about the hints?

---

### Quiz

````quiz
What does `match guess.cmp(&secret_number)` do?

○ It changes the secret number to match the guess
→ Not quite. `cmp` only compares the two numbers. It doesn't change either of them.
◉ It compares the two numbers, then runs the arm that matches the result
→ Yes. `cmp` returns an `Ordering`, and `match` picks the arm for that value.
○ It prints both numbers
→ Not quite. Printing is done inside each arm, with `println!`.
````

````quiz
Why does the line say `let guess: u32`?

○ Because `guess` has to be a string
→ Not quite. The `: u32` makes `guess` a number, not a string.
◉ So Rust knows what kind of number to make when it parses the text
→ Yes. `parse` can produce several kinds of number, so the type tells it which one to use.
○ Because `u32` makes the program run faster
→ Not quite. The type describes what the value is. It doesn't change the speed.
````

````quiz
What does `trim()` do in `guess.trim().parse()`?

○ It turns the text into a number
→ Not quite. That's `parse`, which runs after `trim`.
◉ It removes the newline and spaces from the start and end of the text
→ Yes. Without this, the newline from pressing Enter would stop `parse` from working.
○ It removes all the spaces, including those inside the text
→ Not quite. `trim` only removes the spaces at the start and end.
````

---

*For more detail, see the section on comparing the guess in [The Rust Programming Language](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html).*
