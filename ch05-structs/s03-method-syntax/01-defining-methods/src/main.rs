#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self borrows the instance, so the method can read the fields without taking ownership.
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // Try: change `&self` to `self` in the method. Does the println! below it still compile, and why?
    // println!("{:?}", rect1);
}
