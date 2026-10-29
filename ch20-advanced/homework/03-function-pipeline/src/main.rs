fn double(x: i32) -> i32 {
    x * 2
}

fn increment(x: i32) -> i32 {
    x + 1
}

// Each step is a function pointer. Named functions coerce to fn, so the slice can list them directly.
fn run_fns(steps: &[fn(i32) -> i32], start: i32) -> i32 {
    steps.iter().fold(start, |acc, step| step(acc))
}

// Each step is a boxed closure. The closures can capture values, so each one can be different.
fn run_boxed(steps: &[Box<dyn Fn(i32) -> i32>], start: i32) -> i32 {
    steps.iter().fold(start, |acc, step| step(acc))
}

fn main() {
    println!("fn pipeline: {}", run_fns(&[double, increment], 5));
    println!("reversed: {}", run_fns(&[increment, double], 5));

    // The annotation makes each closure into the same type, Box<dyn Fn>.
    let steps: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x * 10),
        Box::new(move |x| x + 5),
    ];
    println!("boxed: {}", run_boxed(&steps, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_of_steps_matters() {
        assert_eq!(run_fns(&[double, increment], 5), 11);
        assert_eq!(run_fns(&[increment, double], 5), 12);
    }

    #[test]
    fn empty_pipeline_returns_the_start() {
        assert_eq!(run_fns(&[], 7), 7);
    }

    #[test]
    fn boxed_steps_run_in_order() {
        let steps: Vec<Box<dyn Fn(i32) -> i32>> = vec![Box::new(|x| x * 10), Box::new(|x| x - 1)];
        assert_eq!(run_boxed(&steps, 2), 19);
    }
}
