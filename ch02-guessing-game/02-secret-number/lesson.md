## Generate a Secret Number

Now the game has something to guess. The computer needs to choose a secret number between
1 and 100, and it needs to choose a different one each time you play. Rust's standard
library can read input and print text, but it can't make random numbers. For that, we
need a library that someone else has written.

In Rust, a library is called a **crate**. The crate we'll use is called `rand`, and it's
published on a website called crates.io. Think of crates.io as a shared toolbox. Anyone
can publish a crate there, and anyone can download one.

This step is the first time your project depends on code that you didn't write. Cargo
handles the work, but you need to tell it what to download.

---

### Part 1: Add rand to Cargo.toml

Open `Cargo.toml`. Under `[dependencies]`, add a line for `rand`:

```toml
[dependencies]
rand = "0.8.5"
```

This is the project file, so it has no output of its own. The name on the left, `rand`, is the name of the crate. The version on the right is the one
this course is written for. Cargo uses it to find the right version on crates.io. You don't
need to know the details of version numbers yet.

Save the file with **Ctrl+S**.

---

### Part 2: Run it for the first time

This step needs an internet connection, because Cargo has to download `rand` the first time
you build. Press **F5**. The first run takes a little longer than usual. The Output pane will
show lines like these:

```
    Updating crates.io index
     Locking 9 packages to latest compatible versions
   Compiling rand_core v0.6.4
   Compiling rand v0.8.5
   Compiling guessing_game v0.1.0 (C:\VS Code Projects\Rust Training\ch02-guessing-game\02-secret-number)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.12s
     Running `target\debug\guessing_game.exe`
Guess the number!
The secret number is: 42
Please input your guess.
```

Your list of `Compiling` lines will be longer, and the numbers will be different. That's
normal. Cargo is building `rand` and the libraries it depends on, before it builds your own
program.

Type a number in the input box and press **Enter**. The program prints `You guessed:` followed
by your number, as it did before.

---

### Part 3: Run it again

Press **F5** again. This time there are no `Compiling` lines for `rand`, because Cargo already
has them. You'll see something like this:

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target\debug\guessing_game.exe`
Guess the number!
The secret number is: 17
Please input your guess.
```

The secret number is different. Each run starts with a new random number, so you can't
predict it.

---

### Part 4: Read the new code

Here is the program, with comments on the new ideas:

```rust
use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    // gen_range makes a random number in the range 1..=100, which includes both 1 and 100.
    // The Rng trait must be in scope for this method to work, which is what `use rand::Rng` does.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

The output of this program is shown in Parts 2 and 3.

Some of this is new, so let's take it one piece at a time.

**`use rand::Rng;`** brings the `Rng` trait into scope. A *trait* is a set of abilities that
a type can have. The `gen_range` method belongs to that trait, so the program has to name the
trait before it can use the method. If you leave this line out, Rust will tell you it can't
find `gen_range`. That error sounds mysterious, but the fix is always one line like this one.

**`rand::thread_rng()`** gives us a random number generator, which is the source of random
numbers for this thread of the program. You don't need to know how it works inside.

**`.gen_range(1..=100)`** asks the generator for a number in a range. The `1..=100` is a range,
and the `=` matters. Two dots (`1..100`) would mean "from 1 up to, but not including, 100".
Adding the `=` (`1..=100`) includes 100 as well, so the secret number can be any whole number
from 1 to 100.

**`let secret_number = ...`** saves the result in a variable. Because we didn't write `mut`,
the value can't change after this, which is what we want for a secret.

**`println!("The secret number is: {secret_number}");`** prints the secret. This is only here
so you can see that the program works. In the next step, we'll get rid of it, because a
guessing game that shows you the answer isn't much of a game.

---

### Try it

* Run the program several times. Does the secret number change each time?
* Change `1..=100` to `1..=10`, and run it. What range of numbers do you see?
* Change `1..=10` back to `1..=100`. Then change `1..=100` to `1..100`, with no `=`. Can the
  secret ever be 100?

---

### Quiz

````quiz
What is `rand` in this step?

○ A keyword that makes a variable random
→ Not quite. `rand` isn't part of the Rust language. It's a crate, which is a library that someone else has written.
◉ A crate: a library, downloaded from crates.io, that adds features to your program
→ Yes. Cargo downloads crates listed under `[dependencies]` in `Cargo.toml`.
○ A function built into the Rust standard library
→ Not quite. The standard library is included with Rust. `rand` is downloaded separately.
````

````quiz
What does the range `1..=100` include?

○ The numbers 1 to 99 only
→ Not quite. The `=` means the end of the range is included.
◉ Every whole number from 1 to 100, including both 1 and 100
→ Yes. The `=` in `..=` means "up to and including".
○ The numbers 1 to 100, but not 1
→ Not quite. The range starts at 1, and 1 is included.
````

````quiz
Why does the program need `use rand::Rng;`?

○ Because `rand` is the name of the program
→ Not quite. The program's name comes from `Cargo.toml`.
◉ Because the `gen_range` method belongs to the `Rng` trait, and the trait must be in scope to use it
→ Yes. Naming the trait with `use` lets Rust find the method.
○ Because the program can't run without it
→ Not quite. Removing it doesn't stop the program from being built. It causes an error where `gen_range` is used.
````

---

*For more detail, see the section on random numbers in [The Rust Programming Language](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html).*
