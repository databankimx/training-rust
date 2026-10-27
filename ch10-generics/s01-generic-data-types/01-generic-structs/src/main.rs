// Two type parameters let x and y have different types.
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // mixup takes its own types, V and W. The result takes x from self and y from other.
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // Try: change the struct to Point<T> with one type parameter, and build p1 and p2 again. What does the compiler say?
}
