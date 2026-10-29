use averaged_collection::AveragedCollection;

fn main() {
    let mut collection = AveragedCollection::new();
    collection.add(2);
    collection.add(4);
    collection.add(9);
    println!("average: {:.1}", collection.average());

    // The average is updated inside add and remove, so reading it never needs a recalculation.
    let removed = collection.remove();
    println!("removed {:?}, average is now {:.1}", removed, collection.average());

    // Try: call collection.list directly. What does the compiler say, and why?
}
