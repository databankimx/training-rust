#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self: read only. The caller keeps the rectangle.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // &mut self: changes the rectangle. The caller must have a mutable binding.
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // self: takes ownership. The caller can't use the rectangle after this call.
    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

fn main() {
    let mut rect = Rectangle {
        width: 0,
        height: 2,
    };

    println!("{}", rect.area());

    rect.set_width(5);
    println!("{}", rect.area());

    let other = Rectangle {
        width: 1,
        height: 1,
    };
    let max_rect = rect.max(other);
    println!("{:?}", max_rect);

    // Try: uncomment the println! below and run again. Why is it rejected?
    // println!("{}", rect.area());
}
