## Solution: Bank Transfers

This walkthrough explains the example solution in `src/main.rs`. Read it after you've written your own version.

### Run the solution

Press **Run** for the output, and **Test** for the two tests. The output is:

```
balances: [750, 1250]
total: 2000
```

### How it works

Each account is its own `Mutex<i64>`, and the accounts are shared through an `Arc<Vec<...>>`. Each thread gets a clone of the `Arc`, which is a cheap count increase, and
the data itself is not copied.

`transfer` takes the lower-numbered account's lock first, whatever the direction. This is the rule that prevents deadlock. Suppose one thread moved money from account 0 to 1, and
another from 1 to 0. If the first took lock 0 and the second took lock 1, each would wait forever for the other. With the fixed order, both threads take lock 0 before lock 1, so one waits
for the other to finish instead.

Both guards are held while the money moves. That makes the change look atomic to the other threads: nobody can see the money leave one account before it arrives in the other, so the
total never changes.

The tests check the single transfer directly, in both directions, and then check that the total survives. In `main`, the totals come out right whatever the order of the threads.

### The extensions

Checking for insufficient funds should come after taking both locks. If the check came before, another transfer could empty the account between the check and the withdrawal. The
check has to see the same balance that the withdrawal changes.

Locking in the opposite order works in this program only by chance. The transfers are short, so a deadlock rarely happens in practice, and that is what makes it dangerous. A program can
pass its tests and then hang under load. The fixed order removes the possibility, rather than hoping it doesn't occur.

---

*This walkthrough accompanies the exercise in Chapter 16 of the course. It's not from the book.*
