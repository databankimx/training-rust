## Homework: A Workspace

**The problem:** Build a workspace with two packages. A library, `greeter`, provides a function that builds a greeting. A binary, `app`, uses it. The two are developed together
in one workspace.

Work in a folder of your choice, in a terminal, and in VS Code:

1. Create a folder named `greetings`, and a `Cargo.toml` inside it with a `[workspace]` section. Set `resolver` to `"3"`.
2. Create a binary package, `app`, and a library package, `greeter`, inside the workspace, with `cargo new`.
3. In `greeter/src/lib.rs`, write `pub fn greet(name: &str) -> String`, which returns the text `Hello, <name>!`. Add a unit test that checks it.
4. In `app/Cargo.toml`, add `greeter` as a path dependency.
5. In `app/src/main.rs`, call `greet` with your name, and print the result.

Then, from the top-level `greetings` folder, run the app with `cargo run -p app`, and run every test with `cargo test`.

---

### Hints

* The `-p` flag names the package. Without it, Cargo doesn't know which binary to run.
* A workspace has one `Cargo.lock` and one `target` folder, at the top level. Look for them after you build.
* A test in `greeter` runs from the top level with `cargo test`, and `cargo test -p greeter` runs only that package.

---

### Check your work

`cargo run -p app` should print your greeting. `cargo test` should run the test in `greeter`, and show a section for each package.

---

### Extensions

* Add a second library, `farewell`, with a function that says goodbye, and use it from `app` as well.
* Remove `greeter` from `app/Cargo.toml`, and build. What does Cargo say, and why?

---

*This exercise is not from the book. It practices workspaces from Chapter 14.*
