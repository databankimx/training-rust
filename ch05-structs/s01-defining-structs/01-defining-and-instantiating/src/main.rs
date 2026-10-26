struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!(
        "{} signed in {} time(s), active: {}",
        user1.username, user1.sign_in_count, user1.active
    );

    // The whole instance must be mut to change any field.
    user1.email = String::from("anotheremail@example.com");
    println!("new email: {}", user1.email);

    // Try: remove `mut` from user1 and run again. Which line does Rust reject, and why?
}
