pub trait Summary {
    fn author(&self) -> String;

    // A default method. Every type that implements Summary gets this behavior, unless it overrides it.
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.author())
    }
}

struct Post {
    author: String,
}

// Post uses the default summarize.
impl Summary for Post {
    fn author(&self) -> String {
        self.author.clone()
    }
}

struct Tweet {
    user: String,
    text: String,
}

// Tweet overrides summarize, the way a child class might override an inherited method.
impl Summary for Tweet {
    fn author(&self) -> String {
        format!("@{}", self.user)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.author(), self.text)
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let post = Post {
        author: String::from("Alice"),
    };
    let tweet = Tweet {
        user: String::from("bob"),
        text: String::from("Trait objects are handy"),
    };

    println!("{}", post.summarize());
    println!("{}", tweet.summarize());
    notify(&tweet);

    // Try: delete the summarize method inside impl Summary for Tweet. Which of the three lines changes, and to what?
}
