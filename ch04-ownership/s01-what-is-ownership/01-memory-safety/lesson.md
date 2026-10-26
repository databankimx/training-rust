## Memory Safety

This chapter is about ownership, and ownership is a tool for keeping programs *safe*. Before we can talk
about what ownership does, we need to be clear about what "safe" means here. This step is about that
question, and it sets up everything in the rest of the chapter.

Many programmers come to Rust from languages where a wrong step at runtime produces an exception, which you
can catch and handle. Rust takes a different approach, and understanding why is the first step to
understanding ownership.

---

### A program that is safe

Here is a small program that is safe to run:

```rust
fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn main() {
    let x = true;
    read(x);
}
```

Expected output:

```
y is true!
```

The `main` function defines `x` and passes it to `read`. `read` checks whether `y` is true, and prints a
message if it is. Running the program prints `y is true!`, as you'd expect.

---

### A program that is unsafe

Now move the call to `read` above the line that defines `x`:

```rust
fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn main() {
    read(x); // oh no! x isn't defined!
    let x = true;
}
```

This second program is unsafe. The call `read(x)` expects `x` to have a value of type `bool`, but when that
line runs, `x` doesn't have a value yet. Rust refuses to compile it, with an error like this:

```text
error[E0425]: cannot find value `x` in this scope
 --> src/main.rs:8:10
  |
8 |     read(x); // oh no! x isn't defined!
  |          ^ not found in this scope
```

The line numbers may differ. The message tells you what Rust found, and where: it looked for `x` at the point of
the call, and `x` isn't in scope there.

The sample for this step contains this second program, commented out, so you can see the error for yourself.

---

### Why not check at runtime?

Other languages handle this differently. In an interpreted language such as Python, reading a variable before it
is defined raises an exception called `NameError`. In JavaScript it's a `ReferenceError`. The program
starts, reaches the bad line, and fails there.

Those exceptions come at a cost. Each time an interpreted program reads a variable, the interpreter has to check
that the variable is defined. Rust's goal is to produce efficient programs that need as few runtime checks as
possible. So Rust doesn't check at runtime whether a variable is defined before it's used. It checks at
*compile time*, when the program is built, and it refuses to build a program that would fail.

That trade-off is the reason for the rule. But the question remains: why is it important to reject the
program at all? To answer that, we have to ask what would happen if Rust let the program compile.

---

### What would happen if the program ran?

Compilers turn source code into machine instructions. For the safe program, the `main` function's machine code
does roughly this: it places the number 1, which represents `true`, into a register (a small storage location
inside the processor), and then calls `read`, which expects its argument to be in that register.

If the unsafe program were allowed to compile, the machine code would call `read` *before* the value is placed
in that register. The register would then contain whatever was there before, which could be anything: 2, 100, or
a large number that happens to be left over from earlier work. `read` would then treat that value as a `bool`,
even though it isn't one.

This is what the book calls **undefined behavior**. Rust doesn't specify what happens when `if y` runs with a
`y` that is neither `true` nor `false`. Something will happen, but Rust can't tell you what. For example:

* The program might run without any visible problem, and nobody notices.
* It might crash immediately, with a segmentation fault or another operating system error.
* It might run without crashing, until someone supplies just the right input to make it do something harmful.

The last case is not a figure of speech. A program that behaves unpredictably can be exploited, and the consequences
can be serious.

---

### Safety is the absence of undefined behavior

With that in mind, the meaning of safety in Rust becomes clear:

> **A foundational goal of Rust is to ensure that your programs never have undefined behavior.** That is the
> meaning of "safety."

Undefined behavior is especially dangerous in low-level programs, which have direct access to memory. Many
serious security vulnerabilities in low-level software come from memory corruption, which is one kind of
undefined behavior. The Rust Reference keeps a list of
["Behavior considered undefined"](https://doc.rust-lang.org/reference/behavior-considered-undefined.html), and
ownership is about preventing a particular group of those: the operations on memory.

Rust has a secondary goal as well: to catch problems at compile time instead of runtime. This matters for two
reasons. Catching bugs before a program ships means fewer failures in production. And catching bugs at compile time
means fewer runtime checks, which makes the program faster.

---

### What Rust cannot prevent

Rust's protections don't make a program bug-free. If an application exposes a public, unauthenticated endpoint
that deletes a production database, a malicious user doesn't need a clever bug to cause damage. Rust can't stop
that. Its protections are about a specific class of mistakes, the ones that lead to undefined behavior.

Within that class, the protections are strong. Teams that have moved parts of their software to memory-safe
languages have seen large reductions in memory-related security problems, and that's part of why Rust's rules
are worth understanding even when they feel strict.

---

### Two ways to think about memory

Memory is the space where a program keeps its data while it runs. There are many ways to picture it, and two of
them are common:

* A high-level view, where memory is "the RAM in my computer", or "the thing that runs out if I load too much
  data". This is too abstract to explain how Rust works. For example, it doesn't tell you what a pointer is.
* A low-level view, where memory is "an array of bytes", or the addresses you get back from a function such as
  `malloc`. This is too concrete. Rust doesn't let you treat memory as an array of bytes.

Both are valid models of a computer, but neither is useful for understanding Rust's rules. Rust provides its
own model, which the rest of this chapter explains. Ownership is a discipline for using memory safely within
that model.

---

### Try it in the runner

The sample for this step is the safe program. Press **F5** to run it. The Output pane should show:

```
y is true!
```

The comments in `src/main.rs` contain the unsafe version. Uncomment them and run the program. Rust will refuse to
compile it, and the error will be the one described above. Then think about what the error protects you from.

---

*For more detail, see the section "What Is Ownership?" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html).*
