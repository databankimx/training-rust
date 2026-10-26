## 4.3 Fixing Ownership Errors

When the borrow checker rejects your code, you need to decide what to do. This sub-chapter works through common cases, and
the key question in each is the same: is the program actually unsafe, or is the borrow checker being too strict?

* [Returning a reference](#goto/01-returning-a-reference): fixing a reference to data that doesn't live long enough.
* [Not enough permissions](#goto/02-not-enough-permissions): asking only for the permission the function needs.
* [Aliasing a collection](#goto/03-aliasing-a-collection): shortening a borrow so it doesn't overlap a change.
* [Copying and moving out](#goto/04-copying-and-moving-out): why you can't take ownership out of a reference.
* [Safe programs rejected](#goto/05-safe-programs-rejected): when the borrow checker is too cautious, and how to work around it.
* [Summary](#goto/06-summary): the main ideas of this sub-chapter.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
