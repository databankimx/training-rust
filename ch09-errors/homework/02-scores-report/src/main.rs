// Each function can fail, and returns the reason as an Err. The caller combines them with ? and never needs a panic.
fn average(values: &[f64]) -> Result<f64, String> {
    if values.is_empty() {
        return Err(String::from("cannot average an empty list"));
    }
    Ok(values.iter().sum::<f64>() / values.len() as f64)
}

fn percent_of(part: f64, whole: f64) -> Result<f64, String> {
    if whole == 0.0 {
        return Err(String::from("whole is zero"));
    }
    Ok(part / whole * 100.0)
}

fn score_report(scores: &[f64], total: f64) -> Result<String, String> {
    // Each ? passes an error straight back to the caller, so the report is only built when every step succeeds.
    let mean = average(scores)?;
    let percent = percent_of(mean, total)?;
    Ok(format!("mean {mean:.1} is {percent:.1}% of {total}"))
}

fn main() {
    println!("{:?}", score_report(&[8.0, 9.0, 10.0], 10.0));
    println!("{:?}", score_report(&[], 10.0));
    println!("{:?}", score_report(&[1.0], 0.0));
}
