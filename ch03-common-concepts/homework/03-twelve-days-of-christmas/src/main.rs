const GIFTS: [&str; 12] = [
    "a partridge in a pear tree",
    "two turtle doves",
    "three French hens",
    "four calling birds",
    "five golden rings",
    "six geese a-laying",
    "seven swans a-swimming",
    "eight maids a-milking",
    "nine ladies dancing",
    "ten lords a-leaping",
    "eleven pipers piping",
    "twelve drummers drumming",
];

fn main() {
    let ordinals = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];

    for day in 1..=12 {
        print_verse(day, ordinals[day - 1]);
    }
}

fn print_verse(day: usize, ordinal: &str) {
    println!("On the {ordinal} day of Christmas, my true love gave to me:");

    // Count down from this day's gift to the first one.
    for index in (0..day).rev() {
        let gift = GIFTS[index];
        if index > 0 {
            println!("{},", capitalize(gift));
        } else if day == 1 {
            println!("{}.", capitalize(gift));
        } else {
            println!("and {gift}.");
        }
    }

    println!();
}

fn capitalize(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}
