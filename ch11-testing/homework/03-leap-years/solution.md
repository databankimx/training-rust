## Solution: Leap Years

This walkthrough explains the example solution in `src/main.rs`. Read it after you've found and fixed the bug yourself.

### Run the solution

Press **Run** for the program's output, and **Test** for the tests. The output is:

```
2024: true
1900: false
2000: true
2023: false
```

### How it works

The original function checked only divisibility by 4. That's correct for most years, but a century year such as 1900 is divisible by 4 too, so the function wrongly
said it was a leap year.

The fix adds the century rule. A year is a leap year when it's divisible by 4 and not by 100, or when it's divisible by 400. The `||` joins the two cases. The
parentheses matter, because `&&` binds more tightly than `||`, and without them the rules would read differently.

The tests cover each case: an ordinary leap year, a common year, a century that isn't a leap year, and a century that is. The century test is the one that
fails on the original function, which is why writing it first finds the bug.

### The extensions

February's length depends on the leap year rule, so a function for it should call `is_leap_year` rather than repeat the rule. Testing each kind of year keeps
that function honest.

A year of 0 is a question of definition. The calendar has no year zero, so a good answer might be to reject it, or to document that the function accepts it. The
tests should state whichever choice is made.

Changing the type to `i32` needs no test changes, because the tests use only positive years. The remainder operator works the same way for positive numbers in
both types, but negative years would need thought.

---

*This walkthrough accompanies the exercise in Chapter 11 of the course. It's not from the book.*
