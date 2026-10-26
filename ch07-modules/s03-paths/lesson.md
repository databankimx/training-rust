## 7.3 Paths for Referring to an Item in the Module Tree

To use an item, Rust needs its path through the module tree. This sub-chapter covers the two kinds of path, how `pub` makes items reachable,
how `super` moves up a level, and how structs and enums expose their parts.

* [Absolute and relative paths](#goto/01-absolute-and-relative): starting a path at `crate`, or at the current module.
* [Pub and privacy](#goto/02-pub-and-privacy): why a module must be `pub` for its contents to be reached.
* [super](#goto/03-super): reaching the parent module from a child.
* [Public structs and enums](#goto/04-pub-structs-and-enums): making some fields public, and all variants of an enum.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
