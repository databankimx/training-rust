## Solution: Gather Forecasts

This walkthrough explains the example solution in `src/main.rs`. Read it after you've written your own version.

### Run the solution

Press **Run** for the output, and **Test** for the two tests. The output is:

```
Dallas: 88F
Boston: 61F
Seattle: 57F
```

### How it works

`fetch_temperature` waits on its own delay, then returns the city and the temperature. Each call creates a future, and the futures don't run until something awaits them.

`gather_temperatures` maps each request to one of those futures. Every call has the same type, so the iterator can be passed straight to `trpl::join_all`. `join_all` polls all the
futures together, and returns the results in the order the futures were given, not the order they finished. Here Boston finishes first, after 100 ms, but it's printed second,
because it was second in the list.

The total time is the longest delay, 300 ms. Waiting for each request one after another would have taken 600 ms, which is the point of running them together.

The tests check the ordering directly, and check the empty case. Each test calls `trpl::block_on`, because the test itself isn't async.

### The extensions

To give up on a slow request, race it against a timer with `trpl::select`. The timeout from Chapter 17.3 is the same idea, and the request can be wrapped in it.

`trpl::join` takes exactly two futures, and returns a tuple of their two outputs. `join_all` takes any number, and returns a `Vec` of one type. Two requests with different output
types would need `join`, because the `Vec` in `join_all` holds only one type.

---

*This walkthrough accompanies the exercise in Chapter 17 of the course. It's not from the book.*
