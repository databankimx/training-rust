## Homework: Bank Transfers

**The problem:** Two accounts hold money, and several threads move money between them at the same time. Each account is guarded by its own mutex, and the total amount of money
must stay the same, whatever order the transfers happen in.

Create a project in VS Code with `cargo new bank_transfers`. In `src/main.rs`:

* Write `transfer(accounts: &[Mutex<i64>], from: usize, to: usize, amount: i64)`, which takes money out of one account and puts it in another.
* Lock the two accounts in a fixed order, the lower index first, whatever the direction of the transfer. If two threads locked the accounts in opposite orders, each could hold one lock
  and wait forever for the other. This is a deadlock.
* In `main`, share the accounts with `Arc`. Start 5 threads that each move 100 from account 0 to account 1, and 5 threads that each move 50 from account 1 to account 0. Both accounts start
  with 1,000.
* Join every thread, then print the balances and their total.

Write tests in a `tests` module. One checks that a single transfer moves the money. Another checks that a transfer never changes the total.

---

### Hints

* `Arc<Vec<Mutex<i64>>>` shares the accounts. Clone the `Arc` for each thread, and the threads can call `transfer(&accounts, ...)` through the `Arc`.
* To lock in order, compare the two indexes. The lower one gets locked first, and the guards can be held together in a `let`.
* A transfer should take the money out and put it in while both guards are held. If one step happened and the other didn't, the total would change for a moment.

---

### Check your work

Your program should print:

```
balances: [750, 1250]
total: 2000
```

Account 0 starts at 1,000, loses 500 across five transfers of 100, and gains 250 across five transfers of 50, which leaves 750. Account 1 gets the rest.

---

### Extensions

* Reject a transfer that would take an account below zero. Should the check happen before the locks are taken, or after? Why?
* Try locking the accounts in the other order, the higher index first, for the two directions. Does the program ever stop? If it doesn't, you've been lucky.

---

*This exercise is not from the book. It practices mutexes, `Arc`, and deadlock avoidance from Chapter 16.*
