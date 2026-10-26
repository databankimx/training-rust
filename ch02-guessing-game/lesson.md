## Chapter 2: Programming a Guessing Game

In this chapter you build a complete program: a guessing game in which the computer picks a secret number, and you
guess it. The game gives you hints as you go, and you'll use the `rand` library to make the number.

Building the game introduces input, variables, crates, `match`, loops, and handling bad input, all in one place.
The steps follow the order in which the game grows.

* [Guess input](#goto/01-guess-input): read what the player types.
* [Secret number](#goto/02-secret-number): add the `rand` crate and generate the number.
* [Compare guess](#goto/03-compare-guess): tell the player whether the guess is too small, too big, or right.
* [Add a loop](#goto/04-add-a-loop): keep the game going until the player wins.
* [Handle invalid input](#goto/05-handle-invalid-input): ask again when the input isn't a number.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
