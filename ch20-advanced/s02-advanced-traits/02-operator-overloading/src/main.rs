use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Add is declared as Add<Rhs = Self>. Leaving out Rhs means the right-hand side is also a Point.
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

// Here Rhs is set to Meters, so Millimeters can be added to Meters, and the conversion happens inside add.
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + other.0 * 1000)
    }
}

fn main() {
    let p = Point { x: 1, y: 0 } + Point { x: 2, y: 3 };
    println!("{p:?}");

    let total = Millimeters(500) + Meters(2);
    println!("{}mm", total.0);

    // Try: change Meters(2) to Meters(0). What is the total now?
}
