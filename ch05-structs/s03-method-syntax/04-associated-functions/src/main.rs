#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// An associated function has no self parameter. It's called with ::, on the type.
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// A type can have more than one impl block. This one holds the method.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let sq = Rectangle::square(3);

    println!("The area of the square is {}", sq.area());
    println!("{:?}", sq);

    // Try: call Rectangle::square(3).area() directly, without a variable. Does it compile?
}
