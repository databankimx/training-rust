## Solution: Bank Account

This walkthrough explains the example solution in `src/main.rs`. Read it after you've tried the exercise yourself.

### Run the solution

Press **F5**. The Output pane shows:

```
withdrew 30
cannot withdraw 500, balance is 70
balance: 70
```

### How it works

The struct and its `impl` block are inside `mod bank`. The struct is `pub`, so `main` can name it, and the `impl` methods that `main` calls are `pub`.
The field `balance` has no `pub`. That means only code inside `bank` can read or change it. `main` changes the balance through `deposit` and
`withdraw`, and reads it through `balance`.

`use bank::Account;` brings the struct into scope at the crate root, so `main` can write `Account::new()` instead of `bank::Account::new()`.

`withdraw` checks the amount against the balance before it subtracts, so the balance can never go negative. Returning a `Result` makes the
caller deal with the failure: `main` matches on it, and prints either the error or the success.

### The extensions

Storing a history means adding a private field, such as a `Vec` of transactions, and a `pub fn history` that returns a reference to it or a copy. The
field stays private, so callers can only read the history through the method. That keeps the rule that all changes go through `deposit` and `withdraw`.

Writing `acct.balance = 0;` in `main` is rejected, because `balance` is private to `bank`. This is the point of the design. If the field were public,
any code could set the balance to any value, and the checks in `withdraw` would be bypassed.

Moving the `use` inside `main` still works, because `main` is the only code that uses `Account`. A `use` applies only in the scope where it's written, as
the sub-chapter showed.

---

*This walkthrough accompanies the exercise in Chapter 7 of the course. It's not from the book.*
