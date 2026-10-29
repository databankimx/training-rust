## 12.5 Working with Environment Variables

Command line arguments are one way to configure a program. An environment variable is another, and it lasts for the whole terminal session, so a user can set it
once. This sub-chapter adds a case-insensitive search, and uses the `IGNORE_CASE` variable to turn it on.

* [Writing a failing test](#goto/01-failing-test-case-insensitive): the new test, and a skeleton function that compiles but fails.
* [Implementing the case-insensitive search](#goto/02-passing-test-case-insensitive): lowercasing the query and each line.
* [Reading an environment variable](#goto/03-config-and-environment): `env::var`, and choosing the search from the configuration.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
