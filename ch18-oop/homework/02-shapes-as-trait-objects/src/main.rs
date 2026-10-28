use std::f64::consts::PI;

trait Shape {
    fn name(&self) -> String;
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn name(&self) -> String {
        String::from("circle")
    }

    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn name(&self) -> String {
        String::from("rectangle")
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// Takes a slice of trait objects, so any mix of shapes can be totalled.
fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|shape| shape.area()).sum()
}

fn main() {
    // The annotation is needed. Without it, the compiler would infer a single concrete type from the first element.
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Rectangle {
            width: 3.0,
            height: 2.0,
        }),
        Box::new(Circle { radius: 0.5 }),
    ];

    for shape in &shapes {
        println!("{}: {:.2}", shape.name(), shape.area());
    }
    println!("total: {:.2}", total_area(&shapes));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangle_area_is_width_times_height() {
        let rectangle = Rectangle {
            width: 3.0,
            height: 2.0,
        };
        assert_eq!(rectangle.area(), 6.0);
    }

    #[test]
    fn empty_list_has_zero_area() {
        assert_eq!(total_area(&[]), 0.0);
    }
}
