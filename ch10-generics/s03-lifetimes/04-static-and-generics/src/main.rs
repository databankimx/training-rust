use std::fmt::Display;

// Lifetimes and generic parameters share one list. ann is printed, so it needs Display.
fn longest_with_an_announcement<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // String literals have the 'static lifetime, because they are stored in the program itself.
    let s: &'static str = "I have a static lifetime.";
    println!("{s}");

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest_with_an_announcement(string1.as_str(), string2, "Today is a good day");
    println!("The longest string is {result}");

    // Try: change the announcement to a number, such as 2026. Does the function still compile?
}
