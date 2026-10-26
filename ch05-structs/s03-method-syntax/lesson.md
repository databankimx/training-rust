## 5.3 Method Syntax

Methods are functions defined inside an `impl` block, and they're called with dot syntax on an instance. This sub-chapter covers how to
write them, how they borrow or take ownership of `self`, and how they relate to the function calls they replace.

* [Defining methods](#goto/01-defining-methods): the `impl` block, `&self`, and where methods can be defined.
* [Methods with parameters](#goto/02-methods-with-parameters): adding parameters after `self`, such as `can_hold`.
* [Methods and field names](#goto/03-methods-and-field-names): a method that shares a field's name, and getters.
* [Associated functions](#goto/04-associated-functions): constructors, `::` syntax, and multiple `impl` blocks.
* [Method calls are function calls](#goto/05-methods-are-function-calls): how the dot syntax maps onto a function call.
* [Methods and ownership](#goto/06-methods-and-ownership): `&self`, `&mut self`, and `self`, and the permissions each needs.
* [Summary](#goto/07-summary): the main ideas of this sub-chapter.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
