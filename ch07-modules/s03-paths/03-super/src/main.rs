fn deliver_order() {
    println!("Delivered the order");
}

mod back_of_house {
    fn cook_order() {
        println!("Cooked a new order");
    }

    pub fn fix_incorrect_order() {
        cook_order();
        // super starts the path at the parent module, which is the crate root here.
        super::deliver_order();
    }
}

fn main() {
    back_of_house::fix_incorrect_order();

    // Try: change super::deliver_order() to crate::deliver_order(). Does the output change?
}
