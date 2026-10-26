## Handle Invalid Input

At the moment, the game has one fatal weakness. Type `hello` instead of a number, and the
program stops with an error. A player who makes a typing mistake shouldn't lose the game.
In this step, we'll make the program ask again instead.

---

### Part 1: Try to break it

Press **F5**, and when the game asks for a guess, type `hello` and press **Enter**. The
program stops. The Output pane shows a message that begins with `thread 'main' panicked`,
followed by your text, `Please type a number!`. The output looks like this, though the line numbers depend on your version:

```
thread 'main' panicked at src\main.rs:16:...
Please type a number!
```

A *panic* is what Rust does when your program hits a problem it wasn't written to handle. Rust
stops, rather than guess what you meant. Here, it's caused by the `expect` in the last step. That's
useful while you're learning, but it's the wrong response for a player.

Press **F5** again to start a new game.

---

### Part 2: Read the new code

The change is in one place: the line that turns the text into a number. Here it is with
comments on the new ideas:

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // parse() returns a Result, which is either Ok (it worked) or Err (it didn't).
        // match checks which one it is. Ok gives us the number, and Err runs the next line.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // continue skips the rest of this turn and goes back to the top of the loop.
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

The output of this program is shown in Part 3.

The new parts are:

**`match` on a `Result`.** Last step, we used `expect` to turn a failed `parse` into a panic.
Here, `match` lets us decide what to do in each case. `parse()` gives back a `Result`, which is
an enum with two variants: `Ok` and `Err`. You met enums in the comparison step with `Ordering`,
and `Result` works the same way. An `Ok` holds the successful value, and an `Err` holds information
about the failure.

**`Ok(num) => num`** says "if it worked, call the number `num` and use it as the value of the
whole `match`". That value becomes the new `guess`.

**`Err(_) => continue`** handles the failure. The `_` means "I don't need to know what went wrong",
so we don't give the error a name. `continue` then jumps back to the top of the loop, which asks
for another guess. Because the `match` is inside the loop, the player is never sent back to the start.

**`continue` and `break`** are both loop controls. `break` leaves the loop for good, which is what
happens when you win. `continue` only skips to the next turn. You'll see the two side by side
here, one doing each job.

---

### Part 3: Play it

Press **F5** and type a few things that aren't numbers, such as `hello`, `12abc`, and a blank line.
The program should ask for a guess again each time, without printing `You guessed` or
a hint. Then type a number, and the game carries on as before:

```
Guess the number!
Please input your guess.
hello
Please input your guess.
12abc
Please input your guess.

Please input your guess.
50
You guessed: 50
Too small!
```

---

### Try it

* Type `-5`. Is it accepted? Why not? Think about the type `u32`, which only holds whole numbers
  that aren't negative.
* Change `Err(_) => continue,` to `Err(_) => println!("That's not a number. Try again!"),`, and
  remove the `continue`. Rust will refuse to compile the program. Why? Think about what each arm of
  the `match` has to give back.
* Remove the `_` from `Err(_)` and write `Err(e) =>` instead. Rust will warn you that `e` is never
  used. Why would you want the error in a real program, even if this game doesn't need it?

---

### Quiz

````quiz
What does `parse()` give back?

○ A number, or a panic if the text isn't a number
→ Not quite. That was the behavior in the last step, when we added `expect`. `parse` gives back a `Result`, and it's up to you what to do with each case.
◉ A `Result`, which is either `Ok` with the number, or `Err` with an error
→ Yes. Using `match` lets your program choose what to do in each case.
○ A string that contains only the digits
→ Not quite. `parse` turns text into a number. It doesn't return text.
````

````quiz
What does `continue` do inside a loop?

○ It leaves the loop, and the program ends
→ Not quite. That's `break`. `continue` only skips the rest of the current turn.
◉ It skips the rest of this turn and goes straight to the next one
→ Yes. In this game, the player gets another guess.
○ It starts the program again from the beginning
→ Not quite. The program doesn't restart. Only the loop goes back to its top.
````

````quiz
Why does `Err(_)` use an underscore instead of a name?

○ Because `_` makes the program faster
→ Not quite. The underscore only tells Rust we're not going to use the value.
◉ Because we don't need the details of the error, so there's no need to give it a name
→ Yes. The `_` is a placeholder meaning "I'm ignoring this value".
○ Because `Err` can't hold any information
→ Not quite. An `Err` can hold details. Here, we just don't need them.
````

---

*For more detail, see the section on handling invalid input in [The Rust Programming Language](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html).*
