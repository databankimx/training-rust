## 15.4 Rc<T>, the Reference-Counted Smart Pointer

`Rc<T>` lets one value have several owners. It's used when no single part of a program can be known to finish with the data last. This sub-section shares a list between
several others, and watches the reference count change as owners come and go.

* [Sharing data with Rc](#goto/01-sharing-with-rc): three lists that share one tail, with `Rc::clone`.
* [Watching the reference count](#goto/02-reference-counts): `Rc::strong_count`, the drop of a clone, and the book's quizzes.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
