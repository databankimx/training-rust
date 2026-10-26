## Installing Rust

Welcome! Before you can write a single line of Rust, you need a few things on your
computer:

* **VS Code**, the editor you'll write your code in.
* **rustup**, a tool that installs Rust and keeps it up to date.
* The **C++ build tools**, which Rust uses to link your programs on Windows.

Installing them takes a few minutes, and most of the time is spent waiting for downloads.

You will use two Rust commands throughout this course:

* **`rustc`** is the compiler. It turns your code into a program.
* **`cargo`** is the build tool. It compiles, runs, and manages your projects. Most of the
  time, you will use cargo.

---

### Step 1: Install VS Code

Open **PowerShell** (press the Windows key, type `powershell`, and press Enter), then run:

```
winget install Microsoft.VisualStudioCode
```

When it finishes, the last line should say `Successfully installed`. If VS Code is
already installed, winget says so and you can skip this step.

---

### Step 2: Install rustup

In the same PowerShell window, run:

```
winget install Rustlang.Rustup
```

The first time, winget may ask you to agree to some terms. Type `Y` and press Enter.
When it finishes, the last line should say `Successfully installed`.

---

### Step 3: Install the C++ build tools

Rust on Windows needs a linker, which is the part that joins your compiled code into one
program. The linker comes from the Visual Studio Build Tools:

```
winget install Microsoft.VisualStudio.2022.BuildTools
```

The installer opens in its own window, so PowerShell prints nothing more. Choose **Desktop development with C++** and click **Install**.
This is a big download, so this is a good time to make a cup of tea.

> **Linux and macOS:** install rustup with the official script instead:
>
> ```
> curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
> ```
>
> The script asks which installation to use. Press **1** and Enter to accept the defaults.
>
> On macOS, also run `xcode-select --install` to get a C compiler and linker. On Ubuntu,
> install the `build-essential` package.

---

### Step 4: Add the rust-analyzer extension

Open VS Code. Press **Ctrl+Shift+X** to open the Extensions panel, type `rust-analyzer`
into the search box, and click **Install** on the extension published by `rust-lang`.

rust-analyzer checks your code as you type. It underlines mistakes, suggests names, and
tells you what a piece of code means. You'll be glad of it.

---

### Step 5: Open a terminal in VS Code

In VS Code, press **Ctrl+`** (the backtick key, above Tab). A panel opens at the bottom of
the window with a prompt that looks something like this:

```
PS C:\Users\YourName>
```

That is a terminal, and it's where you type commands. Everything in this course that
starts with `rustc` or `cargo` is typed here.

---

### Step 6: Check that it worked

Close the terminal and open a fresh one, so it picks up the new tools. Click the **+**
button in the terminal panel, or press **Ctrl+Shift+`**. Then run:

```
rustc --version
```

You should see something like this:

```
rustc 1.99.1 (58cf8f9c3 2026-10-13)
```

Now check cargo:

```
cargo --version
```

```
cargo 1.99.1 (5f94df478 2026-08-27)
```

Your numbers will probably be different, because Rust releases a new version every six
weeks. That's fine. As long as you see a version number and a date, the installation
worked.

If you see `'rustc' is not recognized`, the new tools aren't on your path yet. Close
VS Code completely, open it again, and try once more. If it still fails, restart your
computer. This is the most common problem, and a restart usually fixes it.

---

### Step 7: Find the documentation

Rust comes with its documentation on your computer, so you can read it without an
internet connection. Run:

```
rustup doc
```

Your web browser should open with the Rust Book, the standard reference, and the library
documentation. Nothing appears in the terminal. When you're done browsing, close the
browser and carry on.

---

### Step 8: Keep it up to date

To update Rust, run:

```
rustup update
```

If you're already up to date, the output says so. Otherwise it lists what it updated, then
finishes.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
What is the name of the command-line tool for managing the version of Rust on your machine?
[ rustup ]
→ rustup installs and updates Rust. rustc is the compiler, and cargo is the build tool.
````

---

*For more detail, see section 1.1 "Installation" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch01-01-installation.html).*
