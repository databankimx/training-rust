## Summary: The State Pattern

The state pattern can be written in Rust in two ways, and they make different trade-offs.

* **Trait objects:** each state is a struct implementing a trait, and the value holds a `Box<dyn State>`. The rules are checked at runtime, and new states are easy to add. *(See the state pattern with trait objects step.)*
* **Types:** each state is its own type, and transitions consume one type to produce the next. Invalid requests fail to compile. *(See the states as types step.)*
* **Choosing:** prefer trait objects when states must be extended by others, and types when mistakes should be caught before the program runs. *(See both steps.)*

An enum is a third option, and the quiz below compares it with the trait version.

---

### Check your understanding

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
In the running example, each state could instead be represented as a variant of an enum, with a `match` in each method. Which of the following is a good reason to prefer the struct/trait representation over the enum?

◉ An API client could add a new state to the system
○ The use of `match` is slower at runtime than dynamic dispatch
○ An API client could add a new method for all states in the system
○ Dynamic dispatch never has a runtime cost
→ Yes. With traits, a client can add a new state by implementing the trait, without changing the existing code. An enum is closed, so a new variant needs a change everywhere it's matched. A `match` is usually
no slower than dynamic dispatch, and the trait version can't add new methods to every state.
````

---

*The summary is written for this course, and draws on section 18.3 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch18-03-oo-design-patterns.html).*
