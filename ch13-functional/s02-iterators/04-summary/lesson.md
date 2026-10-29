## Summary: Iterators

An iterator is a sequence whose items are produced on request, and which does nothing until something asks for them.

* **Using iterators:** `next` returns the next item, or `None` at the end. A `for` loop does the same thing for you. *(See the creating and using step.)*
* **Consuming methods:** `sum` and `collect` take the iterator and use it up, and produce a single result. *(See the consuming and adapting step.)*
* **Adapters:** `map` and `filter` return new iterators, and their closures run only when a consuming method runs the chain. *(See the consuming and adapting step, and
  the filtering with closures step.)*
* **Closures in iterators:** an adapter's closure can capture values from its surroundings, such as the size being searched for. *(See the filtering with closures step.)*

Iterators let you describe a sequence of steps, and the compiler produces a loop as efficient as one you would write by hand. The next sub-section uses them to improve
`minigrep`.

---

*The summary is written for this course, and draws on the summary of section 13.2 in [The Rust Programming Language](https://doc.rust-lang.org/book/ch13-02-iterators.html).*
