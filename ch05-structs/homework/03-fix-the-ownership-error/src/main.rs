#[derive(Debug)]
struct Rectangle {
    name: String,
    width: u32,
    height: u32,
}

impl Rectangle {
    // The fix: borrow self, and build a new rectangle with a copy of the name.
    fn wider(&self, by: u32) -> Rectangle {
        Rectangle {
            name: self.name.clone(),
            width: self.width + by,
            height: self.height,
        }
    }
}

fn grow(rect: &mut Rectangle) {
    *rect = rect.wider(10);
}

fn main() {
    let mut r = Rectangle {
        name: String::from("window"),
        width: 30,
        height: 50,
    };

    grow(&mut r);
    println!("{:?}", r);
}
