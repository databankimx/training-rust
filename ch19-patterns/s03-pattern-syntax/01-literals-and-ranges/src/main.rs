fn describe(n: i32) -> &'static str {
    match n {
        // A literal matches only that value.
        1 => "one",
        // | matches any of several patterns.
        2 | 3 => "two or three",
        // ..= matches an inclusive range.
        4..=9 => "four to nine",
        // _ matches anything else, without binding it to a name.
        _ => "something else",
    }
}

fn main() {
    for n in [1, 3, 7, 12] {
        println!("{n}: {}", describe(n));
    }

    // id @ 40..=49 tests the range, and also binds the matched value to id.
    let message = match 42 {
        id @ 40..=49 => format!("id {id} is in the forties"),
        _ => String::from("other"),
    };
    println!("{message}");

    // Try: change 42 to 52. Which arm runs now, and what does the message say?
}
