#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        // dbg! prints the value to standard error, and then returns it, so width is still 60.
        width: dbg!(30 * scale),
        height: 50,
    };

    // dbg! takes ownership of its argument, so pass a reference to keep rect1 for later use.
    dbg!(&rect1);

    // {:?} uses the Debug format on one line, and {:#?} spreads it over several lines.
    println!("rect1 is {rect1:?}");
    println!("rect1 is {rect1:#?}");

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // Try: remove #[derive(Debug)] and run again. Which line does Rust reject, and what does the note say to add?
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
