## 5.2 An Example Program Using Structs

This sub-chapter shows why structs are worth using. It starts with a simple rectangle-area program that works, but hides what its
values mean. Then it refactors the program in three steps: with a tuple, with a struct, and with `Debug` output for inspecting the struct.

* [Separate variables](#goto/01-separate-variables): a working program whose function takes two unrelated-looking numbers.
* [Refactoring with tuples](#goto/02-refactoring-with-tuples): grouping the numbers, and what the tuple still hides.
* [Refactoring with structs](#goto/03-refactoring-with-structs): naming the group and its parts, and borrowing the struct.
* [Deriving Debug](#goto/04-deriving-debug): printing a struct while you debug it.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
