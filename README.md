# DataBank IMX - Rust Developer Training

## License and Support

This repository is provided under the [MIT License](LICENSE). You are free to use, copy, modify, merge, publish, distribute, sublicense, and/or sell the materials in this repository, subject to the terms of that license.

This repository is provided for training and reference purposes only. DataBank IMX does not provide warranties, guarantees, maintenance, troubleshooting, implementation assistance, or technical support for this code or its use. Use it in any way permitted by the MIT License, but you are responsible for evaluating, adapting, testing, and supporting any use of it in your own environment.

---

## What is this repository for?

- Rust developer training curriculum for DataBank IMX
- Chapter-by-chapter lessons, quizzes, and code samples covering the Rust language from first steps through concurrency, async, and a multithreaded web server, based on *The Rust Programming Language* (the Brown University edition)
- A final project: a calculator built in stages, with a core track and a bonus track, each with a worked example
- Each lesson explains one idea, shows the expected output, and includes a short quiz. Most lessons also include a small Rust program that you can edit and run
- Includes a Tauri-based lesson runner for navigating, reading, and running lessons interactively

---

## About

- **Author:** [Scott McLean](mailto:smclean@databankimx.com)
- **Purpose:** Internal developer training curriculum for DataBank IMX

---

## DataBank Responsibilities for Custom Code

All DataBank employees - including those working with AI-assisted tools - are bound by the [Development Rules of Engagement](development-rules-of-engagement.md). That document defines the rules for quoting, approving, developing, reviewing, scanning, storing, and delivering custom code for client work.

Key obligations that apply to everyone on every custom code engagement:

- **You own the code you deliver.** This applies equally to hand-written code, AI-assisted code, and AI-generated code. "The AI wrote it" is not an acceptable explanation for a production issue, a failed review, a security finding, or a support problem.
- **Code must be reviewed before delivery.** Review is interactive and must confirm that the code works, meets DataBank standards, is maintainable, and that the developer understands it well enough to explain, debug, and maintain it. Polished or well-formatted output - including AI-generated output - is not evidence of correctness.
- **Code must be in source control before delivery.** All custom code must be committed to the appropriate DataBank GitHub Enterprise repository before it is delivered to a client, regardless of whether the client receives source code, compiled binaries, or deployed functionality.
- **CI scanning must pass before delivery.** Required checks are the standards policy scan, SonarQube, and Snyk. A passing local run is not a substitute.
- **All third-party dependencies must be license-vetted.** The license must permit closed-source commercial use. Copyleft or unclear licensing must be escalated before use. Dependencies must not be added simply because an AI tool suggested them.
- **Only approved AI tools may be used.** Use only IT-approved tools through company accounts. All data entered into AI systems must be treated as confidential by default.
- **Custom code is billable work.** Code must be tied to an approved quote or engagement before commitment or delivery. Development quotes require approval from the Development Director or delegated development leadership before being sent externally.
- **Exceptions require explicit approval.** Time pressure alone is not a reason to bypass quote approval, source control, review, or CI scanning.

See the [Development Rules of Engagement](development-rules-of-engagement.md) for the full policy, including billing and quoting rules, source-code delivery rules, the delivery readiness checklist, and the exception process.

---

## Textbook reference

*The Rust Programming Language* - Steve Klabnik and Carol Nichols, with contributions from Chris Krycho and the Rust community - No Starch Press

Textbook resources:
- [The Rust Programming Language, online](https://doc.rust-lang.org/book/) (free, from the Rust project)
- [Brown University edition](https://rust-book.cs.brown.edu/) (the edition this course follows, with extra quizzes and diagrams)
- An offline copy (EPUB) is in the `textbook` folder

### Why The Rust Programming Language?

The Rust Book is the official introduction to the language, and it's written for developers who already know how to program. It covers ownership, which is the idea that sets Rust apart, and then moves on to the tools needed for real projects: error handling, generics, testing, concurrency, async, and a final project that builds a web server. This course follows the book's chapters in order, and adds lessons, quizzes, and homework that check understanding of each one.

---

## Setup and requirements

- Rust 1.85 or later, installed with rustup (the course uses the 2024 edition)
- Node.js LTS, which builds the lesson runner
- The C++ build tools on Windows, which Rust uses for linking. Visual Studio with the *Desktop development with C++* workload also provides them, so you only need the Build Tools if Visual Studio isn't installed
- VS Code with the rust-analyzer extension (recommended), or any editor you prefer

See [QUICKSTART.md](QUICKSTART.md) for a step-by-step guide that sets up everything the lesson runner needs, and tells you how to check that each step worked.

### Quick start

From the repository root, in PowerShell:

```pwsh
powershell -ExecutionPolicy Bypass -File .\setup.ps1
```

The script checks for Rust, the C++ build tools, and Node.js. It installs the lesson runner's packages, and copies the browser libraries the runner needs into place. It's safe to run more than once.

Then start the lesson runner:

```pwsh
cd lesson-runner
npm run tauri dev
```

The first start takes several minutes, because Rust compiles the runner and its dependencies. Later starts are much faster.

### Libraries used by the course

| Library | Where | Purpose | Install |
|---|---|---|---|
| Rust toolchain (rustc, cargo) | Lessons and runner | Builds and runs every lesson program | `winget install Rustlang.Rustup` |
| `tauri` 2 and `tauri-plugin-opener` | Lesson runner (`lesson-runner/src-tauri`) | The desktop window and the link opener | Installed by `cargo` from `Cargo.toml` |
| `serde`, `serde_json`, `open` | Lesson runner | Data handling, and opening links in the browser | Installed by `cargo` from `Cargo.toml` |
| `marked` | Lesson runner | Converts lesson Markdown to HTML | `npm install` in `lesson-runner` (copied to `src/vendor` by the setup script) |
| `@viz-js/viz` | Lesson runner | Draws the diagrams in lessons, using Graphviz | `npm install` in `lesson-runner` (copied to `src/vendor` by the setup script) |
| `@tauri-apps/cli` | Lesson runner | Starts the runner in development mode | `npm install` in `lesson-runner` |
| `trpl` 0.3.0 | Chapters 17 to 21 | The helper crate the async and web server chapters use. It downloads from crates.io on the first run | Installed by `cargo` from each lesson's `Cargo.toml` |

All of these are under permissive licenses (MIT, Apache 2.0, or both), and can be used in closed-source work under the Development Rules of Engagement.

**Vendored files.** The setup script copies `marked` and `@viz-js/viz` into `lesson-runner/src/vendor`. Keep their license and notice files with them.

The full policy is in [Development Rules of Engagement](development-rules-of-engagement.md), section 12.

### VS Code

Install the **rust-analyzer** extension (publisher `rust-lang`). It checks your code as you type, and shows the same errors the compiler does. The setup script installs it when VS Code's `code` command is on your PATH.

---

## Running the training

### Lesson Runner

The lesson runner is a desktop application, built with Tauri, which has a Rust back end and a web-style front end. It lets you browse chapters, read lessons, answer quizzes, and run the code in each step.

Setup is in [QUICKSTART.md](QUICKSTART.md). Once it's set up, start the runner from the `lesson-runner` folder:

```pwsh
npm run tauri dev
```

#### Finding lessons

The chapter list on the left shows each chapter, such as *Ch01 · Intro*. Click a chapter to show its steps, then click a step to open it.

* Chapters are listed in order. The final project comes after Chapter 21.
* Each step is one folder. Steps with a Cargo project have code you can run.
* Homework appears under its chapter, in its own list.
* Drag the edges of the panes to resize them.

#### Reading and running a step

* The **Lesson** tab shows the step's lesson, with quizzes. Answer a quiz, then click **Check answer**. **Show answer** reveals it, and locks the question, as in the Rust Book's quizzes.
* The **Source** tab shows the step's code. The Source pane is an editor. Edits are kept in the runner until you click **Reset**, and nothing is written to the repository.
* Press **Run** (or F5) to run the step. Output appears in the Output pane. Lines starting with `Compiling`, `Finished`, and `Running` are status messages from Cargo, not errors.
* Press **Test** to run the step's tests, for steps that have them.
* If a program asks for input, type in the input box under the Output pane, then press Enter or click **Send**.
* Press **Escape**, or click **Stop**, to stop a program that is still running. While a program runs, the Run button becomes Stop.
* A step with no code has a read-only Source pane, and Run is disabled.

#### Homework and spoilers

Homework steps ask you to write your own program first. The Run button stays disabled until you choose **Show example (spoiler)**. Once you do, the example opens, and Run is available. Try the exercise before you reveal the example, since most of the learning happens while you're stuck.

#### Keyboard shortcuts

| Key | Action |
|---|---|
| F5 | Run the current step |
| Escape | Stop a running program, or cancel the input |

#### Troubleshooting

* **The runner window doesn't open, and the terminal shows an npm error:** run `npm install` again in `lesson-runner`. Check that Node.js is installed with `node --version`.
* **The Lesson tab shows no lessons, and the browser console says a module cannot be found:** the vendored browser libraries are missing. Run the setup script again, or see step 3 of QUICKSTART.md.
* **`rustc` or `cargo` is not recognized:** close the terminal and open a new one. If that doesn't help, restart your computer.
* **The first build is very slow:** this is normal. Rust compiles the runner and its dependencies once.
* **Run shows a linker error:** check that the C++ workload is installed, either in Visual Studio or with the Build Tools, with *Desktop development with C++* chosen.

The runner's source code is in the `lesson-runner/` folder.

### Running lessons directly

Each step with code is a Cargo project. You can run one from a terminal, in the step's folder:

```pwsh
cd ch01-intro\02-hello-world
cargo run
```

Or open the folder in VS Code and use the terminal.

---

## Writing lessons

To add or change lessons, see [AUTHORING.md](AUTHORING.md). It covers how the runner finds chapters, sections, steps, and homework, how to write a lesson and its quizzes, how to build a homework exercise with a spoiler, and the checks to run before a change is committed.

---

## Repository structure

```
training-rust/
|- ch01-intro/                   - Chapter 1: getting started
|- ch02-guessing-game/           - Chapter 2: a guessing game
|- ch03-common-concepts/         - Chapter 3: variables, types, functions, control flow
|- ch04-ownership/               - Chapter 4: ownership, borrowing, slices
|- ch05-structs/                 - Chapter 5: structs
|- ch06-enums/                   - Chapter 6: enums and pattern matching
|- ch07-modules/                 - Chapter 7: packages, crates, and modules
|- ch08-collections/             - Chapter 8: vectors, strings, hash maps
|- ch09-errors/                  - Chapter 9: error handling
|- ch10-generics/                - Chapter 10: generics, traits, and lifetimes
|- ch11-testing/                 - Chapter 11: writing automated tests
|- ch12-io-project/              - Chapter 12: a command-line search tool
|- ch13-functional/              - Chapter 13: closures and iterators
|- ch14-cargo/                   - Chapter 14: more about Cargo and crates.io
|- ch15-smart-pointers/          - Chapter 15: smart pointers
|- ch16-concurrency/             - Chapter 16: fearless concurrency
|- ch17-async/                   - Chapter 17: async and await
|- ch18-oop/                     - Chapter 18: object-oriented features
|- ch19-patterns/                - Chapter 19: patterns and matching
|- ch20-advanced/                - Chapter 20: advanced features
|- ch21-web-server/              - Chapter 21: final project, a multithreaded web server
|- final-project/                - The calculator final project, with homework
|- lesson-runner/                - Tauri lesson runner
|  |- src/                       - Front end: HTML, CSS, JavaScript
|  |- src-tauri/                 - Back end: Rust
|- textbook/                     - Offline copy of the Rust Book (EPUB)
|- setup.ps1                     - Sets up everything the lesson runner needs
|- QUICKSTART.md                 - Step-by-step setup guide
```

Each chapter folder holds its sections, and each section holds its steps. A step is a folder with a `lesson.md`, and a Cargo project if it has code. Steps appear in the order of their names, so the number prefix keeps them in order.

### Part I - The Rust Book, Chapters 1 to 21

| Chapter | Topic |
|---|---|
| `ch01-intro` | Getting started: installing Rust, Hello, World, and Cargo |
| `ch02-guessing-game` | Programming a guessing game |
| `ch03-common-concepts` | Variables, data types, functions, comments, and control flow |
| `ch04-ownership` | Ownership, references and borrowing, and slices |
| `ch05-structs` | Defining and using structs |
| `ch06-enums` | Enums, `match`, and `if let` |
| `ch07-modules` | Packages, crates, and modules |
| `ch08-collections` | Vectors, strings, and hash maps |
| `ch09-errors` | Errors with `panic!`, `Result`, and when to use each |
| `ch10-generics` | Generic types, traits, and lifetimes |
| `ch11-testing` | Writing and running automated tests |
| `ch12-io-project` | A command-line search tool, built step by step |
| `ch13-functional` | Closures and iterators |
| `ch14-cargo` | Cargo profiles, workspaces, and publishing crates |
| `ch15-smart-pointers` | `Box`, `Rc`, `RefCell`, and reference cycles |
| `ch16-concurrency` | Threads, message passing, and shared state |
| `ch17-async` | Futures, streams, and async programming |
| `ch18-oop` | Object-oriented features, trait objects, and the state pattern |
| `ch19-patterns` | Every place patterns can be used, and pattern syntax |
| `ch20-advanced` | Unsafe Rust, advanced traits and types, function pointers, and macros |
| `ch21-web-server` | A multithreaded web server, built with a thread pool |

### Part II - Final Project

| Folder | Topic |
|---|---|
| `final-project/` | A calculator, built in two tracks: a core with precedence, parentheses, and a REPL, and bonus items for exponents, `sqrt`, and extra brackets. Each track has a worked example |

---

## Authorship note

This training curriculum was developed for internal use at DataBank. As with the other training repositories, I used generative AI in places for code review, cleanup, and turning rough notes into readable documentation.

~ Scott McLean, 2026

---

## Version history

| Date | Changes |
|---|---|
| 10/28/2026 | Initial version: lessons and quizzes for chapters 1 to 21 of The Rust Programming Language, the final calculator project with homework, the Tauri lesson runner, QUICKSTART.md, and setup.ps1 |

---

## Contact

- [Scott McLean](mailto:smclean@databankimx.com)
- [Dev Team](mailto:development@databankimx.com)
