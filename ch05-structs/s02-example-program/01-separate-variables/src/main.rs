fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // Try: swap the two arguments in the call to area. Does the program still compile? Does the answer change?
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
