## Homework: Gather Forecasts

**The problem:** Fetch the temperature for several cities at the same time, and print the results in the order the cities were listed, even when the replies arrive in a different order.

Create a project in VS Code with `cargo new gather_forecasts`, then add the crate with `cargo add trpl@0.3.0`. In `src/main.rs`:

* Write `async fn fetch_temperature(city: &str, delay_ms: u64, degrees_f: i32) -> (String, i32)`. It waits for `delay_ms`, using `trpl::sleep`, and returns the city and the temperature.
  The delay stands in for a network request, so the results are predictable.
* Write `async fn gather_temperatures(requests: Vec<(&str, u64, i32)>) -> Vec<(String, i32)>`. It creates one future for each request, runs them together with `trpl::join_all`, and
  returns the results.
* In `main`, call `gather_temperatures` with three cities, and print each as `City: 88F`. Use `trpl::block_on` to run the async function.

Write tests in a `tests` module, using `trpl::block_on` inside each test. Check that the results come back in the order of the requests, not the order of the delays.

---

### Hints

* Map each request to a call of `fetch_temperature`. The futures all have the same type, so they can be collected into one iterator.
* `join_all` takes any iterator of futures, and returns a `Vec` in the order the futures were given.
* Give the cities different delays on purpose. A city with a short delay should still appear in its original position.

---

### Check your work

With the requests `("Dallas", 300, 88)`, `("Boston", 100, 61)`, and `("Seattle", 200, 57)`, your program should print:

```
Dallas: 88F
Boston: 61F
Seattle: 57F
```

The whole program takes about 300 ms, not 600 ms, because the three requests wait together.

---

### Extensions

* Add a request that takes longer than 1 second, and a second version that gives up on it after 1 second, using `trpl::select` with `trpl::sleep`.
* Replace `join_all` with `join` for two requests. What is the output type, and why is it different?

---

*This exercise is not from the book. It practices `join_all` and futures from Chapter 17.*
