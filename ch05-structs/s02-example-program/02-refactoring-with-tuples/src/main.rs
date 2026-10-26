fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );

    // Try: change the tuple to (50, 30). The program still runs. What would go wrong if this were
    // a rectangle drawn on the screen, where width and height mean different things?
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
