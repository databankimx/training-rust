fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");

    // Passing references borrows the strings. main still owns m1 and m2 afterwards.
    greet(&m1, &m2);

    let s = format!("{} {}", m1, m2);
    println!("{s}");

    // Try: change greet so it takes String instead of &String, and call it with greet(m1, m2).
    // Then try to use m1 afterwards. Which kind of error do you see, and why?
}

fn greet(g1: &String, g2: &String) {
    println!("{} {}!", g1, g2);
}
