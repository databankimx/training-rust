trait MakeNoise {
    // A default body. A type can keep it, or override it with its own version.
    fn make_noise(&self) {
        println!("(silence)");
    }
}

struct Dog {}
struct Cat {}

impl MakeNoise for Dog {
    fn make_noise(&self) {
        println!("bark");
    }
}

// Cat has an empty impl block, so it uses the default.
impl MakeNoise for Cat {}

fn main() {
    let dog = Dog {};
    let cat = Cat {};
    dog.make_noise();
    cat.make_noise();

    // Try: delete the make_noise method from Dog's impl block. What does dog.make_noise() print now?
}
