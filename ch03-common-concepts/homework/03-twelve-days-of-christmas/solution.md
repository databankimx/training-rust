## Solution: The Twelve Days of Christmas

This walkthrough explains the example solution in `src/main.rs`. Read it after you've tried the exercise
yourself. Your version may be quite different, and that's fine.

### Run the solution

Press **F5**. The Output pane shows all twelve verses, one after another. The first two are:

```
On the first day of Christmas, my true love gave to me:
A partridge in a pear tree.

On the second day of Christmas, my true love gave to me:
Two turtle doves,
and a partridge in a pear tree.

```

The rest follow the same pattern, counting down from the day's gift to the partridge.

### How it works

The program stores the data in two places. `GIFTS` is a list of the gift phrases, and `ordinals` is a
list of the day names. Both are arrays, so each has a fixed length and each item is found by its index.

`main` is a short `for` loop that visits the days from 1 to 12 and prints each verse:

```rust
for day in 1..=12 {
    print_verse(day, ordinals[day - 1]);
}
```

This fragment is part of the sample program, and prints nothing on its own. The `print_verse` function does the real work, and it's where the repetition is used. It counts down
from the day to 1 using a reversed range, and it picks the gift for each line:

```rust
for index in (0..day).rev() {
    let gift = GIFTS[index];
    // ...
}
```

This fragment is part of the sample, with the body of the loop left out for brevity.

Because the array index counts from zero, the gift for day 3 is `GIFTS[2]`, and the loop variable `index`
runs from 2 down to 0. The `day - 1` in `main` is the same idea for the ordinal names: a day number is one
more than its index. Mixing the two up is an easy mistake, so the program uses each one in just one place.

The `if` and `else if` at the end of the loop decide the punctuation:

* Every gift except the last gets a comma.
* The last gift on the first day gets a full stop, because there's nothing else in that verse.
* The last gift on every other day starts with "and" and ends with a full stop.

The `capitalise` function turns the first letter of each gift into a capital, so `two turtle doves`
prints as `Two turtle doves`.

---

*This walkthrough accompanies the exercise in [The Rust Programming Language](https://rust-book.cs.brown.edu/ch03-05-control-flow.html#summary).*
