## 15.6 Reference Cycles Can Leak Memory

Safe Rust makes memory leaks hard to create, but it doesn't forbid them. A cycle of `Rc` references never reaches a count of zero, so its values are never freed. This
sub-section shows a cycle forming, and then removes it with a `Weak` reference from child to parent.

* [Creating a reference cycle](#goto/01-creating-a-cycle): two lists that point at each other, and the counts that keep them alive.
* [Preventing cycles with Weak](#goto/02-weak-parent-links): a tree whose children refer to their parent weakly, with the book's quiz.
* [Summary](#goto/03-summary): the main ideas of the chapter.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
