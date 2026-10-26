## 4.2 References and Borrowing

References let you use data without taking ownership of it. This sub-chapter covers how references work, the rules that keep
them safe, and how the compiler tracks the permissions a reference uses.

* [References are pointers](#goto/01-references-are-pointers): non-owning pointers, and dereferencing them.
* [Aliasing and mutation](#goto/02-aliasing-and-mutation): why reading and changing the same data at once is dangerous.
* [Permissions](#goto/03-permissions): read, write, and own permissions, and how borrows change them.
* [Mutable references](#goto/04-mutable-references): changing data through a reference.
* [Data must outlive its references](#goto/05-data-must-outlive-references): why references can't outlast the data they point to.
* [Summary](#goto/06-summary): the main ideas of this sub-chapter.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
