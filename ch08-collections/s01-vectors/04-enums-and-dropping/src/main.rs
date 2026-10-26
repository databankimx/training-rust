// Each variant can hold a different type, but every variant is the same enum type, so one vector can hold all of them.
// The values are printed by Debug, which the compiler doesn't count as reading the fields. The allow attribute silences that warning.
#[allow(dead_code)]
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Matching each cell would tell us its kind. Here, Debug prints the variant and its value.
    for cell in &row {
        println!("{cell:?}");
    }

    // Try: add SpreadsheetCell::Int("five") to the vector. What does the compiler say?
}
