struct Item {
    name: String,
    price_cents: u32,
    quantity: u32,
}

impl Item {
    fn new(name: &str, price_cents: u32, quantity: u32) -> Self {
        Self {
            name: String::from(name),
            price_cents,
            quantity,
        }
    }

    // A getter: it returns the name, and does nothing else.
    fn name(&self) -> &str {
        &self.name
    }

    fn total_cents(&self) -> u32 {
        self.price_cents * self.quantity
    }

    // Changing the quantity needs &mut self, so the caller's binding must be mut.
    fn add_quantity(&mut self, extra: u32) {
        self.quantity += extra;
    }
}

fn main() {
    let mut apples = Item::new("apples", 50, 3);

    println!("{}: {} cents", apples.name(), apples.total_cents());

    apples.add_quantity(2);
    println!("{}: {} cents", apples.name(), apples.total_cents());
}
