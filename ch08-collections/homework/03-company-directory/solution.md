## Solution: Company Directory

This walkthrough explains the example solution in `src/main.rs`. Read it after you've tried the exercise yourself.

### Run the solution

Press **F5**, then type the commands from the exercise into the input box. The Output pane shows the program's output, along with each line you type.

### How it works

The `Command` enum has a variant for each command. `Add` uses named fields, so the department and the name can't be confused. The `from_input` function
splits a line into words, and matches them against shapes such as `["Add", name, "to", dept]`. A line that matches no shape gives `None`, and `main` prints
`Input error!`.

The loop in `main` reads lines with `lines`, and turns each one into a command. `Add` calls `entry` with `or_default`, which adds an empty vector for a new
department. Then the name is pushed onto it.

`List` looks up the department with `get`. Because `get` returns a reference, the program clones the names before sorting them, so the directory isn't
changed. An unknown department prints a message, rather than failing silently.

`All` sorts the department names first, and then sorts each department's people. Without the first sort, the order would follow the hash map, which can change
between runs. Sorting both levels gives the same output every time.

`Quit` ends the loop, and the program prints a farewell. Once the loop ends, the program has nothing left to do, so it can stop.

### The extensions

A `Remove` command would find the person in the vector, and remove the entry. If the person isn't there, the program should say so, rather than doing nothing.

A count in `List` is just the length of the vector, which `len` returns.

Printing each person's department in `All` means building a list of pairs, and sorting that list by name. The sorting is the new part: it's done on the whole
company, not one department at a time.

---

*This walkthrough accompanies the exercise in Chapter 8 of the course. It's adapted from the summary of Chapter 8 in the book.*
