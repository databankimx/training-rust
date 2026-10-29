struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // Try: remove the & in the call to area. Rust rejects it, because area would take ownership of rect1.
    // Why does that matter for the println! below it, if you added one?
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
