## Chapter 12: An I/O Project

This chapter builds `minigrep`, a command line search tool, one small step at a time. Along the way it puts together ideas from earlier chapters: modules,
collections, errors, traits, and tests. It also covers how a Rust program reads its arguments, reads files, reports errors, and reads environment variables.

There are no quizzes in this chapter, because it's a hands-on walkthrough. Each sub-chapter adds to the same project.

The chapter is split into sub-chapters:

* [12.1 Accepting command line arguments](#goto/s01-command-line-arguments): collecting the arguments, and saving the two values the search needs.
* [12.2 Reading a file](#goto/s02-reading-a-file): reading the file into a string, and what happens when it can't be read.
* [12.3 Improving the structure and error handling](#goto/s03-improving-the-structure): `Config`, `Result` instead of panics, `run`, and the library crate.
* [12.4 Test-driven development of the search](#goto/s04-test-driven-search): a failing test, the passing `search`, and printing matches.
* [12.5 Working with environment variables](#goto/s05-environment-variables): a case-insensitive search, chosen by `IGNORE_CASE`.
* [12.6 Writing errors to standard error](#goto/s06-standard-error): stdout and stderr, and redirecting output.

---

## Summary

`minigrep` now reads its arguments and an environment variable, reads a file, searches it with a tested function, prints the matches to standard output, and
reports errors to standard error. Its logic lives in a library crate, so the tests can reach it, and `main` only handles the arguments and the exit codes.

Every part of the project uses an idea from earlier chapters. Collections hold the lines and the configuration. `Result` carries errors from the place they happen
to the place they're reported. Lifetimes tie the matching lines to the text they came from. Tests check each part on its own.

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
