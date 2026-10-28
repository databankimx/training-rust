# Authoring Guide

This guide is for anyone adding or changing lessons in `training-rust`. It explains how the lesson runner finds content, how to write a lesson and its quizzes, how to build homework with a spoiler, and what to check before a change is committed.

Read the [README](README.md) first for an overview of the course. For setup, see [QUICKSTART.md](QUICKSTART.md).

---

## How the runner finds content

The lesson runner reads the repository's folders when it starts. It doesn't use a list. Names matter.

| Level | Folder name | Example | Title shown in the runner |
|---|---|---|---|
| Chapter | `chNN-name` at the repository root | `ch17-async` | `Ch17 · Async` |
| Section (optional) | `sNN-name` inside a chapter | `s01-futures-and-syntax` | `Futures and Syntax` |
| Step | `NN-name` inside a chapter or section | `01-hello-async` | `Hello Async` |
| Homework | `homework` folder inside a chapter | `ch17-async/homework/01-gather-forecasts` | Listed under the chapter's homework |
| Extra section | `final-name` at the repository root | `final-project` | `Final Project`, with no chapter number, sorted after Chapter 21 |

Some rules follow from this:

- **Use two digits for numbers,** such as `01`, so that the order is right. Folders are sorted by name.
- **Words are capitalized automatically.** `01-hello-async` shows as `Hello Async`. Acronyms are the exception: the runner keeps a list of them in `lesson-runner/src-tauri/src/discovery.rs`, in `ALL_CAPS_WORDS`. The list includes `api`, `c`, `cli`, `io`, `oo`, `oop`, and `ui`. To show a new acronym in capitals, add it to that list, and rebuild the runner.
- **A folder with neither a `lesson.md` nor a `Cargo.toml` isn't a step.** It's ignored.
- **A step is a folder with a `lesson.md`.** If it also has a `Cargo.toml` and a `src/main.rs`, the runner can run it.

Inside a step, the runner looks for these files:

| File | Used for |
|---|---|
| `lesson.md` | The lesson text, shown in the **Lesson** tab |
| `Cargo.toml` and `src/main.rs` | The code, shown in the **Source** tab, and run by **Run** |
| `src/lib.rs` or `tests/` | Extra source files, shown in a file selector when there are two or more `.rs` files |
| `commands.txt` | Lines that appear as quick picks. Each line is a command, such as `cargo run -- demo` |
| `solution.md` | Homework only. Shown after the example is revealed |
| Any other files | Shown in the runner only if the code uses them, for example `hello.html` |

The **Test** button appears when a step has tests. The runner adds the test button when a step has `#[test]` functions in its source.

---

## Creating a chapter or section

1. Create the folder, using the naming rules above.
2. Add a `lesson.md` to the folder, with an overview of the chapter or section. It should list the sub-chapters, each as a link, so readers can navigate. Link to a step with `[link text](#goto/step-folder-name)`, which the runner turns into a jump.
3. Add the steps, in order.

The overview is short. It says what the section covers, and links to each step, with a sentence about each. The repository's chapter overviews follow this pattern.

---

## Writing a step

Each step explains one idea. A complete beginner should be able to follow it, and should know what to expect.

### The structure of a step

Use this order. Each part is a heading, and sections are separated by a line with three dashes (`---`).

1. **The heading.** A short title, such as `## Reading the Request`.
2. **The explanation.** Introduce the idea, then show the code in a block. Explain what the code does in the sentences around it, not in comments alone.
3. **Expected output.** Show what the program prints, in a plain block:

   ```
   Hello, world!
   ```

4. **Try it in the runner.** A short section that says what to change in `src/main.rs`, and what should happen. The comments in `src/main.rs` should match this section, so the reader can find the `// Try:` comment.
5. **Quiz.** Optional, but most steps that teach a concept include one.
6. **The source.** A line with the book reference, in italics, for example: `*For more detail, see section 19.1 "All the Places Patterns Can Be Used" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch19-01-all-the-places-for-patterns.html).*`

Write in US English. Use hyphens, not em dashes. Don't use emojis.

### Code samples

- **Runnable code** goes in `src/main.rs`, in a Cargo project with `edition = "2024"`. The project's package name is the folder name in snake_case, such as `destructuring_structs`.
- **Snippets** that show part of a program, or that won't compile on purpose, go in a block marked `rust,ignore`:

  ````
  ```rust,ignore
  let x = 5;
  ```
  ````

  The runner shows these, and doesn't try to run them.
- **Output blocks** have no language, so they aren't highlighted as Rust.
- **Try comments.** End `main` with a comment that begins `// Try:`, and asks the reader to change something and predict the result. Then describe the result in the lesson's "Try it in the runner" section.

### Warnings

A step should compile with no warnings. The runner shows warnings in the Output pane, and a warning in a lesson is confusing.

- Name unused variables with a leading underscore, such as `_stream`, when the code is meant to keep the value alive.
- For code that's intentionally unused, such as a struct field that the lesson doesn't read, add `#[allow(dead_code)]` on the item, with a comment that says why.
- In `println!`, write a literal brace as `{{` and `}}`. A single brace is a placeholder, and the compiler will report it.

### Servers and programs that wait

A program that runs until stopped, such as a web server, needs three things in its lesson:

- The runner's **Stop** button ends it. Say so.
- Say what the reader should see in the Output pane, which may be nothing.
- If only one copy can run at a time, because of a port, say so in the lesson, and ask the reader to stop it before the next step.

---

## Writing quizzes

A quiz checks one idea. The runner's format uses four backticks around the block, so code can appear inside it:

````
````quiz
Which keyword declares a variable in Rust?
○ var
→ Rust doesn't have var. It's a keyword from other languages.
◉ let
→ Yes. `let` creates a variable.
````
````

- The question is one line, or a short paragraph and a code block before the quiz block.
- Lines beginning `○` are wrong answers. Lines beginning `◉` are right answers. Each answer is followed by a line beginning `→`, which explains it. Start a correct answer's explanation with `Yes.`, and a wrong answer's with `Not quite.`, so the feedback is consistent.
- **Single answer only.** The runner has no checkboxes. If the source question allows several correct answers, make one yes-or-no question per option, each with its own single answer. The course's design-challenge steps do this.
- **Short-answer questions** give the accepted answer in square brackets, such as `[ main ]`. Use `|` to accept more than one answer.

### Credit for adapted quizzes

Quizzes adapted from the Brown University edition of *The Rust Programming Language* carry a credit line just before them:

```
*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*
```

Write the explanations in your own words. Don't copy the book's explanations.

### Checking a quiz

Read each option as a student would. The right answer shouldn't be the only long or detailed option, and the wrong answers should be plausible. Check the answer by running the code, when the question is about a program's output or whether it compiles. The quiz must match what the runner shows.

---

## Writing homework

Homework is a step where the student writes their own program first. The runner keeps the example solution hidden, until the student reveals it.

Each homework step is a folder in `chNN-name/homework/`, and contains:

| File | Purpose |
|---|---|
| `lesson.md` | The exercise. State the problem, the requirements, and the expected output. Give hints, and a check-your-work section. Don't give the solution. |
| `Cargo.toml` and `src/main.rs` | The example solution. The student doesn't see it until they reveal it. |
| `commands.txt` | Quick picks, such as `cargo run` and `cargo test`. |
| `solution.md` | A walkthrough of the example. Shown after the reveal. Explain the decisions, not each line. |

While the student hasn't revealed the example, the Run button is disabled, and the runner shows the spoiler notice. When they reveal it, Run is enabled, and the walkthrough is available.

A good exercise:

- Has requirements a student can check, and an expected output they can compare with.
- Includes a demo mode or tests where that helps, so the student can check their work.
- Says which parts of the course it practices.
- Includes an extensions section for students who finish early.

Exercises that aren't from the book say so at the bottom of the lesson: `*This exercise is not from the book.*`

---

## Quality checks before committing

Run these checks on every new or changed step.

1. **Build the step.** Run `cargo run` in its folder, or press **Run** in the runner. The output should match the lesson's expected output exactly.
2. **Run the tests.** For steps with tests, run `cargo test`, and check that each test passes.
3. **Check for warnings.** The build should be clean. A warning in the output is a sign that something is off.
4. **Read the lesson aloud.** The explanation should match the code, and the Try section should match the comments.
5. **Check the quizzes.** Each answer should be correct, and only one option should be marked `◉`.
6. **Check the names.** Folder names follow the rules above, and the titles the runner shows read well.
7. **Check the style.** US English, hyphens rather than em dashes, no emojis.
8. **Check for secrets and paths.** No tokens, no personal paths, and no machine-specific settings.

If the step needs a server, a network connection, or a second project, say so in the lesson, before the reader starts it.

---

## Common mistakes

- **A folder name without a number.** Steps without numbers sort unpredictably. Give every step a two-digit number.
- **An expected output that isn't what the program prints.** This is the most common error. Run the step, and copy the output exactly, including spacing and punctuation.
- **A `// Try:` comment that doesn't match the Try section.** Keep them the same.
- **A quiz with two correct answers.** The runner accepts one. Split the question, as described above.
- **A missing `Cargo.toml`.** The runner shows the lesson but can't run the code.
- **A homework solution in the lesson.** The solution belongs in `src/main.rs` and `solution.md`, not in `lesson.md`.

---

*This guide is for the training repository. Questions about the runner itself go to the development team.*
