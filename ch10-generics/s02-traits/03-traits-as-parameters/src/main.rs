pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

// impl Summary accepts any type that implements the trait.
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// The trait bound PartialOrd means T must support >. This fixes the largest function from the last sub-chapter.
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        author: String::from("Iceburgh"),
    };
    notify(&article);

    println!("The largest number is {}", largest(&[34, 50, 25, 100, 65]));
    println!("The largest char is {}", largest(&['y', 'm', 'a', 'q']));

    // Try: call largest with a slice of NewsArticle values. What does the compiler say about the missing bound?
}
