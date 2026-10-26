struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Destructuring a tuple struct names the struct type, then the values.
    let Point(x, y, z) = origin;
    println!("origin is ({x}, {y}, {z})");
    println!("black is ({}, {}, {})", black.0, black.1, black.2);

    let _subject = AlwaysEqual;

    // Try: pass black to a function that takes a Point. Rust rejects it, even though both hold three i32 values.
    // Why does Rust keep these types apart?
}
