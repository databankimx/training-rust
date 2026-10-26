# Rust Training

A self-teaching Rust course. Each lesson explains one idea, shows the expected output,
and gives you a short quiz. Most lessons also have a small Rust program that you can
edit and run from the lesson runner, a desktop app that sits alongside the lessons.

The lessons follow the first three chapters of
[The Rust Programming Language](https://doc.rust-lang.org/book/). The lessons are written
in our own words, and the Rust Book is the place to go for more detail. The book's text
belongs to its authors, Steve Klabnik and Carol Nichols, with Chris Krycho and the Rust
community. An offline copy (EPUB) is in the `textbook` folder.

---

## Contents

- [What you need](#what-you-need)
- [Set up the lesson runner](#set-up-the-lesson-runner)
- [Run the lesson runner](#run-the-lesson-runner)
- [Using the runner](#using-the-runner)
- [Repository layout](#repository-layout)
- [Writing a lesson](#writing-a-lesson)
- [Troubleshooting](#troubleshooting)

---

## What you need

Install these before you start. The lesson runner is built on Windows, and the
instructions below are for Windows. The lessons themselves work on Linux and macOS, and
their commands are noted where they differ.

| Tool | Why | How to install (Windows) |
|---|---|---|
| **Rust** (rustup, rustc, cargo) | Builds and runs the lesson programs | `winget install Rustlang.Rustup` |
| **C++ build tools** | The Rust linker on Windows | `winget install Microsoft.VisualStudio.2022.BuildTools`, then choose *Desktop development with C++* |
| **Node.js** (includes npm) | Builds the lesson runner | `winget install OpenJS.NodeJS.LTS` |
| **VS Code** | Where you read and edit the code | `winget install Microsoft.VisualStudioCode` |

In VS Code, install the **rust-analyzer** extension (publisher `rust-lang`). It checks your
code as you type.

To confirm Rust is installed, open a new terminal and run:

```
rustc --version
cargo --version
```

Each command should print a version number and a date. The lesson
[Installing Rust](ch01-intro/practice/01-installation/lesson.md) covers this in detail.

---

## Set up the lesson runner

The runner is in the `lesson-runner` folder. It's a Tauri app, which means a Rust back
end with a web-style front end. You only need to do the setup steps once, and again if
you move the repository to a new folder.

### 1. Install the runner's packages

Open a terminal in the `lesson-runner` folder and run:

```
cd lesson-runner
npm install
```

npm downloads the packages the runner needs. This takes a minute or two. When it finishes,
the last line should be the number of packages added, with no errors.

### 2. Add the Markdown renderer

The lessons are written in Markdown, and the runner uses a small library called `marked`
to display them. Install it, then copy it into the runner's `src` folder:

```
npm install marked
mkdir src\vendor
copy node_modules\marked\lib\marked.esm.js src\vendor\marked.js
```

The `copy` command prints `1 file(s) copied.` If the file is in a different place, run
`dir node_modules\marked\lib` to find it, and copy that file instead.

> **Linux and macOS:** use `mkdir -p src/vendor` and
> `cp node_modules/marked/lib/marked.esm.js src/vendor/marked.js`.

### 3. Check the layout

After setup, `lesson-runner\src\vendor\marked.js` should exist. If it doesn't, the Lesson
tab will not show any lessons, and the browser console will say that the module cannot be
found.

---

## Run the lesson runner

From the `lesson-runner` folder, run:

```
npm run tauri dev
```

The first run takes several minutes, because Tauri and its Rust dependencies are compiled
for the first time. Later runs are much faster. When it's ready, a window titled
**lesson-runner** opens, with the chapters on the left.

The terminal keeps running while the window is open. To close the runner, close its
window. If the terminal is still busy afterwards, press **Ctrl+C** in it.

---

## Using the runner

### Choosing a lesson

* The **chapter list** at the top left shows each chapter, such as *Ch01 · Intro*. Click a
  chapter to show its steps.
* The **steps list** below it shows the steps in the selected chapter. Click a step to open
  it.
* Drag the edges of the panes to resize them. You can drag the sidebar's right edge, the
  boundary between the chapter list and the step list, and the boundary above the Source
  and Lesson pane.

### Reading a lesson

Click the **Lesson** tab, in the lower pane, to read the lesson. Lessons include
expected outputs and quizzes. Links to web pages open in your default browser.

### Working through a quiz

Each quiz asks one question.

* Choose an answer, or type one in short-answer questions, then click **Check answer**.
* A wrong answer shows an explanation and a **Try again** button.
* **Show answer** reveals the answer. Once you've shown an answer, that question is locked,
  as in the Rust Book's quizzes.
* Your answers are kept when you move between lessons.

### Running a program

Click the **Source** tab to see the code. Then:

* Press **F5**, or click **Run**, to run the step's program.
* The **Output** pane shows what the program prints. Lines starting with `Compiling`,
  `Finished`, and `Running` are status messages from Cargo, not errors.
* If the program asks for input, type in the box at the bottom of the Output pane, then
  press Enter or click **Send**.
* Press **Escape**, or click **Stop**, to stop a program that is still running.

### Editing code

The Source pane is an editor. Change the code, then press **F5** to run your changes. The
runner runs a copy of the step, so the files in the repository are not changed. Click
**Reset** to go back to the original code. A **modified** label shows when your code
differs from the file on disk.

If a step has no code, its Source pane is read-only and the Run button is disabled.

### Other buttons

| Control | What it does |
|---|---|
| Sun and moon switch | Switches between dark and light themes. The lesson window follows the main window. |
| Copy icon (Output) | Copies the output |
| Copy icon (Source or Lesson) | Copies the text of the tab you are on |
| Pop-out icon | Opens the lesson in its own window, so you can read it while you code |
| Dock icon | Returns a popped-out lesson to the main window |

---

## Repository layout

```
training-rust/
├── ch01-intro/                  Chapter 1: Getting started
│   ├── 01-installation/         A lesson with no code
│   ├── 02-hello-world/          A lesson with a Cargo project
│   └── 03-hello-cargo/          A lesson with a Cargo project
├── lesson-runner/               The lesson runner (Tauri app)
│   ├── src/                     Front end: HTML, CSS, JavaScript
│   └── src-tauri/               Back end: Rust
├── textbook/                    Offline copy of the Rust Book (EPUB)
└── README.md                    This file
```

Each chapter folder holds its steps. Each step is one folder, and every step has a
`lesson.md`. A step that has code also has a `Cargo.toml` and a `src/main.rs`.
Chapters and steps appear in the runner in the order of their names, so prefix step
folders with numbers, such as `01-installation`.

---

## Writing a lesson

To add a step:

1. Create a folder in the chapter's folder, named with a number and a short name, such as
   `04-variables`.
2. Add a `lesson.md`. Write it for a complete beginner: explain each command, show the
   expected output, and say what to do if something goes wrong.
3. If the step has code, add a Cargo project in that folder: a `Cargo.toml` and a
   `src/main.rs`. Use `edition = "2024"`.

Quizzes go in the lesson as a fenced block, using four backticks so that code can appear
inside them:

``````
````quiz
Which keyword declares a variable in Rust?
○ var
→ Rust doesn't have var. It's a keyword from other languages.
◉ let
→ Yes. `let` creates a variable.
````
``````

* Lines starting with `○` are wrong choices, and lines starting with `◉` are the right
  choice.
* A line starting with `→` explains the choice above it.
* For a short-answer question, give the answer in square brackets, such as `[ main ]`.
  Use `|` to accept more than one answer.

---

## Troubleshooting

**The runner window doesn't open, and the terminal shows a `npm` error.**
Run `npm install` again in `lesson-runner`. If it still fails, check that Node.js is
installed with `node --version`.

**The Lesson tab says a module cannot be found.**
The Markdown renderer is missing. Repeat step 2 of the setup.

**`rustc` or `cargo` is not recognized.**
Close the terminal and open a new one. If that doesn't help, restart your computer. The
installer adds Rust to your path, and an open terminal doesn't see the change.

**The first build is very slow.**
This is normal. Rust compiles the runner and its dependencies the first time you run it.

**Run does nothing, or the Output pane shows a linker error.**
Check that the C++ build tools are installed, with *Desktop development with C++* chosen.

**A step shows the wrong code after I edited it.**
Click **Reset** in the Source pane. Your edits are only held in the runner while it's open,
and Reset returns the step to the file on disk.

---

*Rust Training is a course for beginners. Corrections and suggestions are welcome.*
