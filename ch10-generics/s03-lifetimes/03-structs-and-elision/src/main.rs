// The struct holds a reference, so it needs a lifetime. An excerpt can't outlive the text it points into.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // Elision applies here: &self gives the output its lifetime, so no annotation is needed.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago, I went to sea.");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{}", excerpt.announce_and_return_part("Listen up"));

    // Try: change the method to take announcement: &'static str, and return a string literal instead of self.part.
}
