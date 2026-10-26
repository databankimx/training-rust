## Homework: Bank Account

**The problem:** Write a bank account in a module called `bank`. The account keeps its balance private, so code outside the module can only
change it through methods. Give the `Account` struct these methods:

* `new() -> Account`, which makes an account with a balance of zero.
* `deposit(&mut self, amount: i64)`, which adds to the balance.
* `withdraw(&mut self, amount: i64) -> Result<(), String>`, which takes money out, unless the amount is more than the balance. In that case it
  returns an error that says how much is left.
* `balance(&self) -> i64`, which returns the balance.

Then, outside the module, bring `Account` into scope with `use`. In `main`, deposit 100, withdraw 30, try to withdraw 500, and print the balance.

Try writing it yourself before you look at the example. Start with the module, and make sure the balance field is not `pub`.

---

### Hints

* Put the struct and its `impl` block inside `mod bank { ... }`. Only mark the items `main` needs as `pub`.
* The balance field should stay private. If `main` needs to see the balance, give it a method, not a `pub` field.
* `withdraw` should return `Ok(())` on success, and `Err` with a message when the balance is too low. Check the amount before you subtract.
* In `main`, match on the `Result` from `withdraw` to decide what to print.

---

### Test your code

Your program should print these three lines:

```
withdrew 30
cannot withdraw 500, balance is 70
balance: 70
```

---

### Extensions

* Add a `pub fn history(&self) -> ...` that returns the list of deposits and withdrawals. Where would you store them, and which items should be private?
* Try writing `acct.balance = 0;` in `main`. What does the compiler say, and why does that matter for the design?
* Move the `use bank::Account;` line inside `main`. Does it still work? What does that show about scope?

---

*This exercise is not from the book. It practices modules, privacy, and `use` from Chapter 7.*
