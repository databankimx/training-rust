#[derive(Debug, Clone, Copy)]
enum Light {
    Red,
    Green,
    Yellow,
}

impl Light {
    // Takes self by value. Light is Copy, so the caller can keep using its own copy.
    fn next(self) -> Light {
        match self {
            Light::Red => Light::Green,
            Light::Green => Light::Yellow,
            Light::Yellow => Light::Red,
        }
    }

    fn seconds(&self) -> u32 {
        match self {
            Light::Red => 30,
            Light::Green => 25,
            Light::Yellow => 5,
        }
    }
}

fn main() {
    let mut light = Light::Red;

    for _ in 0..4 {
        println!("{:?} for {} seconds", light, light.seconds());
        light = light.next();
    }
}
