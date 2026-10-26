## Add a Loop

Right now the game lets you guess once, then it stops. That's not much of a game. A real
guessing game keeps going until you find the number. In this step, we'll wrap the guessing
in a *loop*, so the program asks again after every guess and only stops when you win.

You haven't compiled this version yet. Press **F5** to see what the compiler says about it
the first time. It'll compile, because it's correct, but the first build takes a moment.

---

### Part 1: Play it

Press **F5** and make a few guesses. Your output will look something like this:

```
Guess the number!
Please input your guess.
50
You guessed: 50
Too big!
Please input your guess.
25
You guessed: 25
Too small!
Please input your guess.
37
You guessed: 37
You win!
```

The program keeps asking until you guess the secret number, then it stops by itself.

---

### Part 2: Read the new code

The loop wraps everything that happens on each turn. Here it is with comments on the new ideas:

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // loop repeats the code inside its curly brackets, again and again, until break is used.
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            // break leaves the loop. Without it, the game would never end.
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

The output of this program is shown in Part 1.

The new parts are:

**`loop { ... }`** runs the code inside the brackets over and over. Every time it reaches the
closing bracket, it goes back to the top. Rust doesn't stop a `loop` on its own, so the
program would run forever unless something tells it to stop.

**`break;`** is that something. It leaves the loop immediately. It's inside the `Ordering::Equal`
arm, so it only runs when you win.

Look at the structure of the program now. The steps are the same as before, in the same order:
ask for a guess, read it, compare it. The loop simply makes them repeat. Moving code into a
loop is usually the first change you make when a program needs to keep going.

Notice that `let mut guess = String::new();` is now inside the loop too. That's on purpose. Each
turn needs a fresh, empty `guess` to read into. If the `String` were outside the loop, your old
guesses would still be there, and `read_line` would add to them.

---

### Try it

* Guess the secret number, and count how many guesses you need. Can you do it in seven or fewer?
  Binary search, which you might have met elsewhere, can always do it in seven.
* Remove `break;` and run the program. What happens when you guess correctly? Stop it with
  **Escape** when you've seen enough.
* Move `let mut guess = String::new();` above `loop {`. Play a few turns. What do you notice about
  the output?

---

### Quiz

````quiz
What does `loop { ... }` do?

○ It runs the code inside the brackets once, then stops
→ Not quite. Without a loop, the code runs once and stops. `loop` runs the code again and again.
◉ It runs the code inside the brackets again and again, until `break` leaves the loop
→ Yes. Without `break`, a `loop` never stops on its own.
○ It repeats the code only for a fixed number of times, which you choose
→ Not quite. That's a `for` loop, which you'll meet in a later chapter. A plain `loop` keeps going until `break`.
````

````quiz
What does `break;` do inside a loop?

○ It pauses the loop until the player presses a key
→ Not quite. Nothing in this program waits for a key press except the input line.
◉ It leaves the loop, so the code after the loop runs
→ Yes. In this game, nothing comes after the loop, so the program ends.
○ It skips to the next turn of the loop
→ Not quite. That's `continue`. `break` leaves the loop completely.
````

````quiz
Why is `let mut guess = String::new();` inside the loop?

○ Because Rust only allows variables inside loops
→ Not quite. Variables can be declared anywhere a block of code allows.
◉ So each turn starts with a new, empty `guess`, rather than adding to the last one
→ Yes. `read_line` adds to whatever is already in `guess`, so the value must be reset each turn.
○ To make the program faster
→ Not quite. The position of the line changes what the program does, not how fast it runs.
````

---

*For more detail, see the section on loops in [The Rust Programming Language](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html).*
