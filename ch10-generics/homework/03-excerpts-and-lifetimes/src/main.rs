// An Excerpt borrows its text, so it can't outlive the String it points into.
struct Excerpt<'a> {
    text: &'a str,
}

impl<'a> Excerpt<'a> {
    // Elision gives the result the lifetime of &self, so no annotation is needed.
    fn first_word(&self) -> &str {
        self.text.split_whitespace().next().unwrap_or("")
    }
}

// The result borrows from one of the two inputs, so both must share the lifetime 'a.
fn pick<'a>(first: &'a str, second: &'a str, prefer_second: bool) -> &'a str {
    if prefer_second {
        second
    } else {
        first
    }
}

fn main() {
    let book = String::from("Call me Ishmael. Some years ago...");
    let excerpt = Excerpt { text: &book };
    println!("first word: {}", excerpt.first_word());

    let title = String::from("Moby");
    println!("{}", pick(&title, excerpt.first_word(), true));
    println!("{}", pick(&title, excerpt.first_word(), false));
}
