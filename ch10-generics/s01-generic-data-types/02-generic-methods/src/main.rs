struct Point<T> {
    x: T,
    y: T,
}

// Methods here work for a Point of any type, because T is declared right after impl.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// This block applies only to Point<f32>. Other Point types don't get distance_from_origin.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p: Point<f32> = Point { x: 3.0, y: 4.0 };

    println!("x = {}", p.x());
    println!("distance = {}", p.distance_from_origin());

    // Try: call distance_from_origin on a Point<i32>. What does the compiler say?
}
