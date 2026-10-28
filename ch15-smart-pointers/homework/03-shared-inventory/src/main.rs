use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    // One inventory, shared by two owners. The Weak handle doesn't count as an owner.
    let shared = Rc::new(RefCell::new(Vec::<String>::new()));
    let warehouse = Rc::clone(&shared);
    let store = Rc::clone(&shared);
    let watcher: Weak<RefCell<Vec<String>>> = Rc::downgrade(&shared);

    warehouse.borrow_mut().push(String::from("apples"));
    store.borrow_mut().push(String::from("pears"));

    println!("items: {:?}", shared.borrow());
    println!("owners: {}", Rc::strong_count(&shared));

    drop(shared);
    drop(warehouse);
    println!("owners after dropping shared and warehouse: {}", Rc::strong_count(&store));

    drop(store);
    match watcher.upgrade() {
        Some(_) => println!("inventory still exists"),
        None => println!("inventory is gone"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weak_handle_upgrades_while_an_owner_exists() {
        let shared = Rc::new(RefCell::new(Vec::<String>::new()));
        let watcher = Rc::downgrade(&shared);
        assert!(watcher.upgrade().is_some());
    }

    #[test]
    fn weak_handle_is_gone_after_the_last_owner_is_dropped() {
        let shared = Rc::new(RefCell::new(Vec::<String>::new()));
        let watcher = Rc::downgrade(&shared);
        drop(shared);
        assert!(watcher.upgrade().is_none());
    }
}
