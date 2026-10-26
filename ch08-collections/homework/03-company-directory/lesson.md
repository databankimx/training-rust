## Homework: Company Directory

**The problem:** Build a directory of a company's people, grouped by department. The user types commands, and the program updates the directory and
answers them:

* `Add <name> to <department>` adds a person to a department. For example, `Add Sally to Engineering`.
* `List <department>` prints the people in one department, sorted alphabetically.
* `All` prints every department in alphabetical order, with its people sorted.
* `Quit` ends the program.

If a line doesn't match any command, the program says `Input error!`. If a department is unknown, `List` says so.

The directory is a hash map, where each department name maps to a vector of people:

```rust,ignore
type Directory = HashMap<String, Vec<String>>;
```

Try writing it yourself before you look at the example. Start with a `Command` enum, a function that turns a line into a command, and a loop in `main`
that reads lines until `Quit`.

---

### Hints

* To read lines, use `io::stdin().lock().lines()`. You need `use std::io::BufRead` for `lines` to be available.
* Each line can be split into words with `split_whitespace`. A slice pattern like `["Add", name, "to", dept]` matches a line with that shape.
* `add` can use `entry` with `or_default`, which adds an empty vector for a new department, as in Chapter 8.3.
* Sort copies of the vectors before printing, so the directory itself isn't changed.

---

### Test your code

Type these lines, one at a time, and press Enter after each:

```
Add Sally to Engineering
Add Amir to Sales
Add Bob to Engineering
List Engineering
All
Quit
```

Your program should print this:

```
Engineering: Bob
Engineering: Sally
Engineering: Bob
Engineering: Sally
Sales: Amir
Have a nice day!
```

The first two lines come from `List Engineering`, and the next three from `All`. The `Add` lines print nothing, but the lines you type still appear in the Output pane as you enter them. Check the order by hand: `Bob` comes
before `Sally`, and `Engineering` comes before `Sales`.

---

### Extensions

* Add a `Remove <name> from <department>` command. What should happen when the person isn't in the department?
* Make `List` say how many people are in the department, as well as listing them.
* Print each person's department after their name in the `All` output, and keep the output sorted by name across the whole company.

---

*This exercise is adapted from the summary of Chapter 8 in [The Rust Programming Language](https://doc.rust-lang.org/book/ch08-03-hash-maps.html).*
