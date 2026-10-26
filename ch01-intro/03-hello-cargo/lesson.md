## Hello, Cargo!

In the last lesson, you compiled a program by hand with `rustc`. It worked, which is always
nice. It also meant typing two commands, remembering the file names, and hoping you hadn't
left anything out. That's a bit like cooking every dinner from a list you wrote on the back
of an envelope.

**Cargo** is the grown-up version. It builds your program, runs it, fetches any libraries
your program needs, and keeps track of where everything is. Almost every Rust project uses
it, including the ones in this course. You'll be typing `cargo` a lot, so it's worth getting
comfortable with it now.

The good news is that you don't have to remember much. You need four commands, and
Cargo's messages will tell you what it's doing.

---

### Part 1: Let Cargo make the project

Open a terminal in VS Code with **Ctrl+`**. Go to your Rust Training folder, then ask Cargo
to create a new project:

```
cd "C:\VS Code Projects\Rust Training"
cargo new hello_cargo
```

Cargo should respond with this:

```
    Creating binary (application) `hello_cargo` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

The first line is what matters. Cargo has made a folder called `hello_cargo`, which holds a
program that prints a greeting. The second line is a helpful link, and you don't need to
open it today.

If `cd` says `Cannot find path`, create the `C:\VS Code Projects\Rust Training` folder in
File Explorer first, then try again.

> **Linux and macOS:** use the same folder under your home directory, for example
> `cd "~/VS Code Projects/Rust Training"`. The `cargo` commands are the same.

---

### Part 2: Open the project in VS Code

Move into the new folder and open it:

```
cd hello_cargo
code .
```

A new VS Code window opens. In the Explorer panel you'll see this:

```
hello_cargo
├── .gitignore
├── Cargo.toml
└── src
    └── main.rs
```

Cargo made each of these for a reason:

* **`Cargo.toml`** describes the project. It's written in a format called TOML, which is
  mostly `name = value` lines, grouped under headings in square brackets. Open it, and you
  should see something like this:

  ```toml
  [package]
  name = "hello_cargo"
  version = "0.1.0"
  edition = "2024"

  [dependencies]
  ```

  This is the project file, so it has no output of its own. `[package]` is the heading for facts about the project. `[dependencies]` is where you
  would list any libraries your project uses. This project doesn't use any yet, so the list
  is empty.

* **`src`** is the folder for your source code. Cargo expects your Rust files to live here.
* **`src/main.rs`** is the program itself. Open it, and you'll see the same Hello, world!
  program you met in the last lesson.
* **`.gitignore`** lists files that version control should ignore. You can leave it alone.

---

### Part 3: Change the message

Open `src/main.rs` and change the text in the quotes, so the file reads:

```rust
fn main() {
    println!("Hello, Cargo!");
}
```

This program's output is shown in Part 4.

Save with **Ctrl+S**. Nothing has run yet. You've only changed the source code. The next
command runs it.

---

### Part 4: Run it

In the terminal, make sure you're still in the `hello_cargo` folder, then run:

```
cargo run
```

The first time, Cargo has more to do, so you'll see something like this:

```
   Compiling hello_cargo v0.1.0 (C:\VS Code Projects\Rust Training\hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.83s
     Running `target\debug\hello_cargo.exe`
Hello, Cargo!
```

Read it from the top:

* `Compiling` means Cargo is turning your source code into a program.
* `Finished` means the compiling is done. The `dev` profile means this is a quick, unpolished
  build for development. Don't worry about the rest of that line for now.
* `Running` shows the program Cargo just ran.
* `Hello, Cargo!` is your program, doing what you asked.

> **Linux and macOS:** the output is the same, except the program path is shown with `/`
> instead of `\`, and the program has no `.exe` extension.

Now run it again without changing anything:

```
cargo run
```

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target\debug\hello_cargo.exe`
Hello, Cargo!
```

There's no `Compiling` line this time. Cargo noticed that nothing has changed, so it skipped
straight to running the program. That's Cargo being efficient, and it's one reason Cargo is
worth using.

---

### Part 5: Check without building

Sometimes you only want to know whether your code is correct. For that, use:

```
cargo check
```

```
    Checking hello_cargo v0.1.0 (C:\VS Code Projects\Rust Training\hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
```

`cargo check` goes through your code and reports any errors, but it doesn't make a program.
It's much quicker than `cargo build`, so many Rust programmers run it constantly while they
work.

---

### Part 6: Build without running

If you want the program file but don't want to run it yet, use:

```
cargo build
```

```
   Compiling hello_cargo v0.1.0 (C:\VS Code Projects\Rust Training\hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
```

Cargo puts the program in a folder called `target`. Look in `target\debug` and you'll find
`hello_cargo.exe`. You can run it directly with `.\target\debug\hello_cargo.exe`.

---

### Part 7: Build a release version

When you're happy with a program and want it to run as fast as possible, build a release
version:

```
cargo build --release
```

```
   Compiling hello_cargo v0.1.0 (C:\VS Code Projects\Rust Training\hello_cargo)
    Finished `release` profile [optimized] target(s) in 0.62s
```

This takes longer, because Cargo spends more effort making the program fast. The result
goes into `target\release`, and it's the one you would give to someone else.

---

### Summary

| Command | What it does |
|---|---|
| `cargo new name` | Creates a new project |
| `cargo run` | Builds the program if needed, then runs it |
| `cargo check` | Checks the code, without making a program |
| `cargo build` | Builds the program, without running it |
| `cargo build --release` | Builds an optimised program for release |

---

### In the runner

When you press **F5** on this step, the runner runs `cargo run` on the copy of this project
in the step's folder. The output looks the same as above. The runner shows the message from
Cargo, so you can see what it did.

---

### Try it

* Change the message in `println!` again. Run `cargo run`, and notice that Cargo compiles
  again before it runs.
* Make a deliberate mistake: delete the closing `}` at the end of the file. Run `cargo check`
  and read the error. Then put the brace back.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Say you just downloaded a Cargo project, and then you run `cargo run` at the command-line. Which statement is NOT true about what happens next?

○ Cargo downloads and builds any dependencies of the project
→ True. If a project lists libraries in `[dependencies]`, Cargo fetches and builds them first.
○ Cargo builds the project into a binary in the `target/debug` directory
→ True. Debug builds go into `target\debug` on Windows, or `target/debug` elsewhere.
○ Cargo executes the project's binary
→ True. That's the "run" in `cargo run`.
◉ Cargo watches for file changes and re-executes the binary on a change
→ This is the one that isn't true. Cargo doesn't watch your files. You run `cargo run` again each time you want a new build.
````

---

*For more detail, see section 1.3 "Hello, Cargo!" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html).*
