pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// The caller knows only that the result implements Summary. The concrete type stays hidden.
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    }
}

fn main() {
    let post = returns_summarizable();
    println!("{}", post.summarize());

    // Try: change the return type to SocialPost, and run again. Does the output change?
}
