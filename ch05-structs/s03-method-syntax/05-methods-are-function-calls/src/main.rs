struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn main() {
    let mut r = Rectangle {
        width: 1,
        height: 2,
    };

    // The method call and the function call are the same operation.
    let area1 = r.area();
    let area2 = Rectangle::area(&r);
    println!("{area1} {area2}");

    r.set_width(2);
    Rectangle::set_width(&mut r, 2);
    println!("{}", r.area());

    // Try: replace &r in the call to Rectangle::area with r. What does the compiler say?
}
