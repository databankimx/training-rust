use std::time::Instant;

fn search_loop<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

fn search_iter<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

fn main() {
    // About 4 MB of text, so the timing is larger than the noise of a single run.
    let line = "the quick brown fox jumps over the lazy dog\n";
    let contents = line.repeat(100_000);
    let query = "the";

    let start = Instant::now();
    let loop_results = search_loop(query, &contents);
    let loop_time = start.elapsed();

    let start = Instant::now();
    let iter_results = search_iter(query, &contents);
    let iter_time = start.elapsed();

    println!("loop: {} matches in {:?}", loop_results.len(), loop_time);
    println!("iterator: {} matches in {:?}", iter_results.len(), iter_time);
    println!("same results: {}", loop_results == iter_results);

    // Try: run the program in a terminal with cargo run --release. How do the two times change?
}
