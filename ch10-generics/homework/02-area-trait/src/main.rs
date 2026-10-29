// A trait with one required method, and a default method built on it.
trait Area {
    fn area(&self) -> f64;

    fn describe(&self) -> String {
        format!("area {:.2}", self.area())
    }
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}

// Rectangle keeps the default describe method.
impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// T: Area means the function works for any type that has an area, and it can call area on each item.
fn largest<T: Area>(items: &[T]) -> Option<&T> {
    items.iter().fold(None, |best: Option<&T>, item| match best {
        Some(b) if b.area() >= item.area() => Some(b),
        _ => Some(item),
    })
}

fn main() {
    let circles = vec![Circle { radius: 1.0 }, Circle { radius: 2.0 }];
    let rectangles = vec![
        Rectangle { width: 3.0, height: 4.0 },
        Rectangle { width: 2.0, height: 10.0 },
    ];

    println!("{}", largest(&circles).unwrap().describe());
    println!("{}", largest(&rectangles).unwrap().describe());
}
