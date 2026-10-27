## 12.3 Improving the Structure and Error Handling

This sub-chapter refactors `minigrep` in small steps. Each step keeps the program working, so the changes can be checked as they're made. The steps separate the
argument handling from the work, group the configuration, report errors clearly, and move the logic into a library.

* [Grouping the configuration](#goto/01-config-struct): the `Config` struct, and `parse_config` moved out of `main`.
* [Returning a result instead of panicking](#goto/02-building-config): `Config::build`, and a clear message for missing arguments.
* [Moving the logic into run](#goto/03-running-the-logic): `run` returns a `Result`, and `main` reports its errors.
* [Splitting code into a library crate](#goto/04-library-crate): `src/lib.rs`, and why the logic belongs there.
* [Summary](#goto/05-summary): the main ideas of this sub-chapter.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
