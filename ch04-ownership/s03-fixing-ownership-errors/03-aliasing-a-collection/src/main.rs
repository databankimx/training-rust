fn main() {
    let mut dst = vec![String::from("Ferris"), String::from("Rustacean")];
    let src = vec![String::from("Ok"), String::from("Crab"), String::from("Rust Crab Gopher")];

    add_big_strings(&mut dst, &src);
    println!("{:?}", dst);

    // Try: change largest_len back to a reference, `let largest: &String = dst.iter()...;`, and keep
    // the push inside the loop. Which line does the borrow checker reject, and why?
}

fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}
