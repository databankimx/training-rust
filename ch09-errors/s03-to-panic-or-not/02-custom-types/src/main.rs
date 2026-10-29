mod guessing_game;

use guessing_game::Guess;

fn main() {
    // A valid guess is created, and its value is read back through the getter.
    let guess = Guess::new(42);
    println!("Your guess is {}", guess.value());

    // Try: change 42 to 200 and run again. What does the panic message say?
}
