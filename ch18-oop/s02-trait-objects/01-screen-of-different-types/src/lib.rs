// The library defines the Draw trait and a Screen that holds any number of Draw values, so it never needs to know their concrete types.
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a {}x{} button labeled '{}'", self.width, self.height, self.label);
    }
}
