## Homework: Garden Privacy

**The problem:** The program below calls two functions that the compiler won't let `main` reach. Fix it, by deciding which items should be
public, and which should stay private.

```rust,ignore
mod garden {
    pub mod vegetables {
        fn fertilizer() -> &'static str {
            "compost"
        }

        pub fn plant(name: &str) -> String {
            format!("planted {name} with {}", fertilizer())
        }
    }

    fn season() -> &'static str {
        "summer"
    }

    pub fn harvest() -> String {
        format!("harvested in {}", season())
    }
}

fn main() {
    println!("{}", garden::vegetables::plant("beans"));
    println!("{}", garden::harvest());
    println!("It is {}", garden::vegetables::fertilizer());
    println!("It is {}", garden::season());
}
```

The compiler reports that `fertilizer` and `season` are private. The problem is not the paths. It's a question of design: should `main` be able
to reach those items at all?

Try fixing it yourself before you look at the example.

---

### Hints

* Ask what `fertilizer` is for. It's a detail of how `plant` works. Should a caller need it directly?
* Ask whether a caller needs the season. It's useful to know, and `harvest` already uses it.
* For each item you decide to expose, add `pub`. For each one you keep hidden, make sure `main` doesn't call it.
* Items in `garden` can use `season` and `fertilizer` from inside their own module, whatever their visibility.

---

### Test your code

Your program should print these three lines:

```
planted beans with compost
harvested in summer
It is summer
```

---

### Extensions

* Make `fertilizer` public as well. Does it still make sense to keep `season` public? Which item is the real public interface of `garden`?
* Write `pub(crate) fn season()` instead of `pub fn season()`. What changes if this were a library used by another crate?
* Move `harvest` into `vegetables`, so it calls `super::season()`. Does the path change, and does the program still work?

---

*This exercise is not from the book. It practices privacy and `pub` from Chapter 7.*
