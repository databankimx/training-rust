struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Field init shorthand: a field called `email` can take the parameter `email` without repeating it.
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    // Struct update syntax: take the remaining fields from user1.
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{} {}", user2.email, user2.username);
    println!("active: {}, sign-ins: {}", user2.active, user2.sign_in_count);

    // Try: print user1.active and user1.sign_in_count after creating user2. Why do these still work,
    // while user1.username does not?
}
