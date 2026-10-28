## 15.5 RefCell<T> and the Interior Mutability Pattern

`RefCell<T>` lets a value be changed through an immutable reference. The borrowing rules are checked when the program runs, not when it compiles. This sub-section shows the
rules at runtime, builds a mock object that needs interior mutability in its tests, and combines `RefCell<T>` with `Rc<T>` to share mutable data.

* [Borrowing rules checked at runtime](#goto/01-borrowing-at-runtime): `borrow` and `borrow_mut`, and what breaks the rules.
* [Testing with a mock object](#goto/02-mock-messenger): a `LimitTracker` library, and a `RefCell`-based mock in its tests.
* [Multiple owners of mutable data](#goto/03-multiple-owners): `Rc<RefCell<T>>`, and the book's quizzes.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
