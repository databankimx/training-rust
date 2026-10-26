fn main() {
    let name = vec![String::from("Ferris"), String::from("Jr.")];
    println!("{}", stringify_name_with_title(&name));

    // Try: change the signature to take &mut Vec<String> and push "Esq." inside the function.
    // Does main still compile without changes? Should the caller expect its vector to change?
}

fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}
