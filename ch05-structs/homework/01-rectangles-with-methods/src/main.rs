#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let sq = Rectangle::square(4);

    println!("rect perimeter: {}", rect.perimeter());
    println!("rect is square: {}", rect.is_square());
    println!("sq perimeter: {}", sq.perimeter());
    println!("sq is square: {}", sq.is_square());
}
