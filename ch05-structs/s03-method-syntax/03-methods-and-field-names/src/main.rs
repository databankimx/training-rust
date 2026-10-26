struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // A method can share a name with a field. This one returns true when the field is above zero.
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // With parentheses, Rust calls the method. Without them, it reads the field.
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("The rectangle is {} pixels high", rect1.height);

    // Try: remove the parentheses after rect1.width in the if. What does the compiler say?
}
