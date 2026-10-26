## Hello, world!

Every programming course has a Hello, world! program, and this one is no different. It's
short, it's a bit silly, and it's the first time your computer will do something because
you asked it to. Let's make it happen.

By the end of this lesson, you will have:

* made your own folder and file in VS Code,
* compiled it with `rustc` and run it,
* and seen what happens when you make a mistake, which you will do on purpose.

---

### The program

This is the whole program. You'll type it in yourself in a moment.

```rust
fn main() {
    println!("Hello, world!");
}
```

The output of this program is shown in Part 7.

---

### Part 1: Make a folder

Open a terminal in VS Code: press **Ctrl+`**. Then run these three commands, one at a time.
Each one is typed at the prompt and followed by Enter.

```
cd "C:\VS Code Projects\Rust Training"
mkdir hello_world
cd hello_world
```

* `cd "C:\VS Code Projects\Rust Training"` moves into your Rust Training folder. This is the
  folder we suggest for all your Rust projects. If you've chosen a different folder, use its
  path instead, and use the same folder for the rest of the course.
* If `cd` says `Cannot find path`, the folder doesn't exist yet. Create it in File Explorer
  (for example `C:\VS Code Projects\Rust Training`), then run `cd` again.
* `mkdir hello_world` makes a new folder called `hello_world`. A successful `mkdir` prints
  nothing at all. Silence is a good sign.
* `cd hello_world` moves into the new folder.

Your prompt should now end with `hello_world>`, like this:

```
PS C:\VS Code Projects\Rust Training\hello_world>
```

> **Linux and macOS:** use the same folder under your home directory, for example
> `cd "~/VS Code Projects/Rust Training"`, and `mkdir hello_world` and `cd hello_world`
> work the same way.

---

### Part 2: Open the folder in VS Code

Type this in the terminal and press Enter:

```
code .
```

`code .` means "open VS Code, in this folder". A new VS Code window opens with
`hello_world` shown in the Explorer panel on the left. The panel is empty, because the
folder is empty. That's about to change.

If `code .` says it isn't recognized, use **File, Open Folder** in VS Code instead, and
choose `hello_world` in your Documents folder.

---

### Part 3: Create the file

In VS Code, click the **New File** icon in the Explorer panel. It looks like a page with a
plus sign, and it appears when you hover over the folder name. Type this name, and press
Enter:

```
main.rs
```

Rust source files always end in `.rs`. The `main` part is only a name. The compiler will
build any file you give it, but `main.rs` is the usual name for a program's first file, and
the examples in this course use it.

The file opens in the editor, and `main.rs` appears in the Explorer panel.

---

### Part 4: Type the program

Click in the editor and type the program exactly as it appears above. Some things to know
before you start:

* **VS Code adds closing brackets and quotes for you.** When you type `{`, it adds `}`
  after the cursor. When you type `"`, it adds a second `"`. Keep typing and the extra
  ones get pushed along. If you finish with two closing brackets, delete the extra one.
* **Capital letters and punctuation matter.** `println` is not the same as `Println`, and a
  missing `;` will cause an error. Don't worry, that's part of the lesson.
* **Let VS Code suggest names, but don't accept them by accident.** If a box of suggestions
  appears while you type, press **Esc** to close it. Press **Enter** only if you want the
  suggestion.

When you've finished, the file should match the program above. Save it with **Ctrl+S**.

---

### Part 5: Check the file is there

In the terminal, run:

```
dir
```

You should see `main.rs` in the list. It's the only file so far:

```
    Directory: C:\VS Code Projects\Rust Training\hello_world

Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a----        10/26/2026     09:14             67 main.rs
```

Your date, time, and file size will differ. What matters is that `main.rs` is there.

> **Linux and macOS:** use `ls` instead of `dir`.

---

### Part 6: Compile it

Now ask the Rust compiler to turn your code into a program. Run:

```
rustc main.rs
```

If your code is correct, the compiler prints nothing and you get your prompt back. Silence
means success.

Run `dir` again. You should now see two new files next to `main.rs`:

```
main.exe
main.pdb
```

`main.exe` is your program. `main.pdb` is extra information that helps with debugging on
Windows. You can ignore it for now.

> **Linux and macOS:** compiling works the same way, but the program is called `main` with
> no `.exe`, and there is no `.pdb` file.

---

### Part 7: Run it

Now run the program:

```
.\main.exe
```

The `.\` means "in this folder". You should see this:

```
Hello, world!
```

If you see that, you have written and run your first Rust program. Take a moment. It's
allowed.

> **Linux and macOS:** run it with `./main` instead of `.\main`.

---

### What each part means

Here's your program again, one piece at a time:

* **`fn main()`** declares a *function* called `main`. A function is a named block of code.
  Every Rust program starts running at `main`, so every program needs one.
* **`{` and `}`** mark where the function's body starts and ends. The code between them
  belongs to `main`.
* **`println!("Hello, world!");`** prints the text in the quotes, followed by a new line.
  - `println` is a *macro*, which is a bit like a function. The `!` at the end tells you
    it's a macro. You don't need to know more about macros yet.
  - `"Hello, world!"` is a *string*: text, wrapped in double quotes.
  - `;` ends the statement. Almost every line of Rust that does something ends with one.

---

### Part 8: Make a mistake on purpose

Mistakes are where most of the learning happens, so let's make one.

In VS Code, delete the `;` at the end of the `println!` line, then save with **Ctrl+S**. You'll
see a red squiggle appear, because rust-analyzer has spotted the problem. Now run:

```
rustc main.rs
```

The compiler refuses to make a program. You should see an error that begins something
like this:

```
error: expected `;`, found `}`
```

The exact wording and the line numbers may differ between Rust versions, but the first line
is always the most useful part. It tells you what the compiler expected, and where it was
looking. Here, it expected a semicolon after `println!(...)`, and it found the closing brace
instead.

Put the `;` back, save, and run `rustc main.rs` again. Silence means success. Then run
`.\main.exe` once more to be sure.

---

### In the runner

The runner has its own copy of this program, in the folder `02-hello-world`. When you press
**F5**, the runner builds it with cargo and runs it. You'll see something like this:

```
   Compiling hello_world v0.1.0 (C:\Development\training\training-rust\ch01-intro\practice\02-hello-world)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.83s
     Running `target\debug\hello_world.exe`
Hello, world!
```

The lines starting with `Compiling`, `Finished`, and `Running` are status messages from
cargo. They are not errors. The line that matters is `Hello, world!`.

---

### Try it

* Change the text inside the quotes, then compile and run again. Does the new text appear?
* Add a second `println!` line below the first, with a different message. Which line
  prints first?
* Put a `//` in front of a `println!` line. Rust treats everything after `//` as a comment,
  so the line won't run. Try it, then take the `//` away again.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Every executable Rust program must contain a function with the name:
[ main ]
→ Rust always starts a program at the function called main. Its name is fixed.
````

````quiz
Let's say you have the following program in a file `hello.rs`:

```rust
fn main() {
    println!("Hello world!");
}
```

Say you then run the command `rustc hello.rs` from the command-line. Which statement best describes what happens next?

○ `rustc` executes the program and prints out `Hello world!`
→ Running a program is a separate step. `rustc` only compiles, which means it turns your code into a program file. Then you run that file.
○ `rustc` will print an error because this is an invalid program
→ The program is valid. It has a `main` function, and the statement ends with a semicolon, so it compiles cleanly.
◉ `rustc` generates a binary executable named `hello`
→ Yes. `rustc` turns `hello.rs` into a program called `hello`, which is `hello.exe` on Windows.
○ `rustc` reformats `hello.rs` according to the Rust style guide
→ Formatting is a different tool, called `rustfmt`. `rustc` only compiles.
````

---

*For more detail, see section 1.2 "Hello, World!" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch01-02-hello-world.html).*
