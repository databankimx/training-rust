## Homework: The Twelve Days of Christmas

**The problem:** Print the lyrics to the Christmas carol "The Twelve Days of Christmas," taking advantage of
the repetition in the song.

The song is built from one verse, repeated over and over, with one more gift added each time. If you find
the pattern, you won't need to write twelve separate verses.

Try writing your own version before you look at the example.

---

### Hints

* Look at the words of the song closely. Each verse starts with the day, then lists the gifts from that
  day back to the first one. The gifts are the same every time.
* The gifts are a list of words. A list of values of the same type, with a fixed length, is what an array
  is for.
* The ordinal words, such as "second" and "third", are also a list. Think about how to get the right one for
  each day.
* Punctuation matters in the song. Most lines end with a comma, and the last line of each verse ends with a
  full stop. Think about which line is the last one in each verse, and what comes before it.
* The first verse is different from the others. The partridge at the end starts with "and" in every verse
  except the first.

---

### Test your program

Check the first two verses against the song. Your program should print this, as the expected output for the first two verses:

```
On the first day of Christmas, my true love gave to me:
A partridge in a pear tree.

On the second day of Christmas, my true love gave to me:
Two turtle doves,
and a partridge in a pear tree.
```

Then check that your program prints all twelve verses, and that the last verse ends with "twelve drummers
drumming" before the partridge.

---

### Extensions

* Ask the player how many days to sing, and stop at 12.
* Put the gifts in a different order, and see how the verses change. Does the song still make sense?
* Try writing the song without an array of gifts, using a `match` on the day. Which version is easier to
  change?

---

*This exercise comes from the summary at the end of Chapter 3 in [The Rust Programming Language](https://rust-book.cs.brown.edu/ch03-05-control-flow.html#summary).*
