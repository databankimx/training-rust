# Quickstart

This guide takes you from a fresh copy of the repository to a working lesson runner, where every lesson example can run. Follow the steps in order.
Each step says how to check it worked. The last section covers the errors you're most likely to see.

Commands are for **Windows PowerShell**, and assume you're in the repository root, `training-rust`, unless a step says otherwise.

---

## Quick setup with the script

`setup.ps1` installs what's missing, clones the repository if you don't have it yet, and then checks or does steps 1 to 7 below, in order. It's safe to run more than once: anything already in place is skipped.

Open **PowerShell as Administrator**, go to the folder where you saved `setup.ps1`, and run:

```pwsh
powershell -ExecutionPolicy Bypass -File .\setup.ps1 -InstallMissing
```

The `-ExecutionPolicy Bypass` part lets this one script run. It doesn't change any setting on your machine.

With `-InstallMissing`, the script:

- installs **Rust** (rustup and the stable toolchain), **Node.js LTS**, and **Git**, if they're missing
- installs the **C++ build tools** silently, if Visual Studio with the C++ workload isn't already installed. This is a large download, and it can take several minutes
- clones the repository into `C:\Development\training\training-rust`, if the script isn't already inside it, and continues from the clone
- installs the runner's packages, copies the browser libraries into place, and downloads the runner's Rust crates

The elevated window is needed for the C++ build tools, which install machine-wide. Without `-InstallMissing`, the script doesn't install anything. It stops and shows the winget commands to run. Company policy can block winget, in which case the script says so and you'll need IT.

When the script finishes, every line in the summary should say `ok`. Then start the runner yourself (step 7), so you can see its first build.

---

## 1. Install the tools

| Tool | Why | Check |
|---|---|---|
| **Rust** (rustup, rustc, cargo) | Builds and runs every lesson program, and the lesson runner | `rustc --version` shows 1.85 or later |
| **C++ build tools** | Rust's linker on Windows. Only needed if Visual Studio with the C++ workload isn't already installed | Installed with the *Desktop development with C++* workload |
| **Node.js LTS** | Builds the lesson runner, and supplies the browser libraries it shows lessons with | `npm --version` prints a version |
| **VS Code** (optional) | Editor, with the rust-analyzer extension | |
| **Git** (optional) | Updating the repository | `git --version` |

Install them with winget, in an elevated (Administrator) PowerShell window. The C++ build tools install silently, and take several minutes:

```pwsh
winget install Rustlang.Rustup
rustup default stable
winget install Microsoft.VisualStudio.BuildTools --override "--quiet --wait --norestart --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
winget install OpenJS.NodeJS.LTS
winget install Microsoft.VisualStudioCode
```

`rustup default stable` is needed after installing rustup. The rustup install only adds the manager, and the stable toolchain provides `rustc` and `cargo`. Open a new PowerShell window before you run it, so rustup is on the PATH.

The C++ build tools are only needed if you don't already have Visual Studio with the *Desktop development with C++* workload. Visual Studio includes the same
linker, so a full Visual Studio install with that workload is enough, and you can skip the Build Tools command above. The setup script checks for either one.

The build tools install is large, and can take a while. If you prefer, install the **Visual Studio Build Tools** from Microsoft, and choose *Desktop development with C++*.

Restart PowerShell after installing, so the new PATH is picked up.

---

## 2. Check Rust

Open a new PowerShell window, and run:

```pwsh
rustc --version
cargo --version
```

**Check:** both commands print a version, with a date. The version of `rustc` should be 1.85 or later, because the course uses the 2024 edition.

If `rustc` isn't recognized, close the window and open a new one. If that doesn't help, restart the computer.

To update Rust later, run `rustup update`.

---

## 3. Install the lesson runner's packages

The lesson runner is a Tauri app, with a Rust back end and a web-style front end. Its JavaScript packages go in `lesson-runner\node_modules`:

```pwsh
cd lesson-runner
npm install
```

npm downloads the packages, which takes a minute or two. The last line should show the number of packages added, with no errors.

**Check:** these two folders exist:

- `lesson-runner\node_modules\marked`
- `lesson-runner\node_modules\@viz-js\viz`

---

## 4. Copy the browser libraries into place

The lesson pages load two libraries from `lesson-runner\src\vendor`. `marked` turns the Markdown lessons into HTML, and `viz` draws the diagrams. Copy them from `node_modules` into that folder, still in `lesson-runner`:

```pwsh
copy node_modules\marked\lib\marked.esm.js src\vendor\marked.js
copy node_modules\@viz-js\viz\dist\viz.js src\vendor\viz.js
```

Each `copy` command prints `1 file(s) copied.`

**Check:** these two files exist:

- `lesson-runner\src\vendor\marked.js`
- `lesson-runner\src\vendor\viz.js`

If either is missing, the Lesson tab shows no lessons, and the browser console says a module can't be found. Repeat this step.

Then go back to the repository root:

```pwsh
cd ..
```

---

## 5. Download the runner's Rust crates

The runner's Rust packages are listed in `lesson-runner\src-tauri\Cargo.toml`. Download them now, so the first start doesn't wait on the network:

```pwsh
cd lesson-runner\src-tauri
cargo fetch
cd ..\..
```

**Check:** the command finishes with no errors. It doesn't print much when it works.

The first `npm run tauri dev` still compiles the runner, which takes several minutes. Downloading now only saves the network time.

---

## 6. Set up VS Code (optional, recommended)

Install the **rust-analyzer** extension (publisher `rust-lang`). It checks your code as you type. From a PowerShell window:

```pwsh
code --install-extension rust-lang.rust-analyzer
```

Then open the repository root (not a sub-folder) in VS Code.

**Check:** the Extensions view lists rust-analyzer as installed, and opening a step's `src\main.rs` shows no red errors for a correct program.

---

## 7. Start the lesson runner

From the `lesson-runner` folder:

```pwsh
cd lesson-runner
npm run tauri dev
```

The first start takes several minutes, because Rust compiles the runner and its dependencies. When it's ready, a window titled **lesson-runner** opens, with the chapters on the left. Leave the terminal open while the window is running. To close the runner, close its window. If the terminal is still busy, press **Ctrl+C** in it.

---

## 8. Your first run of each kind of step

Work through these once, to confirm each kind of example runs.

| Kind | Where | What you should see |
|---|---|---|
| Program with output | Ch01, step 2 (Hello, World) | `Hello, world!` in the Output pane. Press **Run** (or F5) |
| Program that asks for input | Ch02, Guessing Game | The Output pane asks for a guess. Type a number in the input box under the Output pane, then press Enter |
| Tests | Ch11, a step with a Test button | Press **Test**. The Output pane shows the test results |
| Crates from the internet | Ch17, step 1 | The first run downloads the `trpl` crate, which needs an internet connection. It can take a minute |
| A web server | Ch21, step 1 | Press **Run**, then open `http://127.0.0.1:7878` in a browser. Press **Stop** in the runner before you run another server step |
| Homework | Final Project, Calculator core | **Run** is disabled until you press **Show example (spoiler)**. Try the exercise first |

**The web server steps** only run one at a time, because each one uses port 7878. Stop the server before you start the next one. A browser makes extra connections, so if a server step seems to stop early, use the PowerShell `curl.exe` commands that the Chapter 21 lessons give.

---

## Troubleshooting

| What you see | What to do |
|---|---|
| `rustc` or `cargo` is not recognized | Close the terminal and open a new one. If that doesn't help, restart the computer. |
| `rustc --version` shows a version before 1.85 | Run `rustup update`, then open a new terminal. |
| `npm` is not recognized | Install Node.js LTS (step 1), then open a new terminal. |
| The Lesson tab shows no lessons, and the browser console says a module can't be found | The browser libraries are missing. Repeat step 4. |
| `npm install` fails with a network error | Check your connection or proxy settings. Company policy can also block npm, so ask IT if the error persists. |
| Run shows a linker error, such as `link.exe not found` | Install the C++ workload, either in Visual Studio or with the Build Tools (step 1), then restart the computer. |
| The first build is very slow | This is normal. Rust compiles the runner and its dependencies once. Later builds are much faster. |
| Chapter 17 says it can't download a crate | The first run of a step with `trpl` needs an internet connection. Check your connection, then press **Run** again. |
| A web server step can't start, or says the address is in use | Another server is still running on port 7878. Press **Stop** in the runner, then run the step again. |
| A step shows the wrong code after you edited it | Click **Reset** in the Source pane. Edits are kept in the runner while it's open, and Reset returns the step to the file on disk. |
| `setup.ps1` is blocked from running | Use the full command in the quick setup section, which includes `-ExecutionPolicy Bypass`. |

If a problem isn't listed here, note the step you were on and the full error text, then report it. Include the Output pane text, because the first error is
often the useful one.
