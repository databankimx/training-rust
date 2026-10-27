struct TestResult {
    /// Student's scores on a test
    scores: Vec<usize>,
    /// A possible value to curve all scores
    curve: Option<usize>,
}

impl TestResult {
    /// If there is a curve, then increments all scores by the curve
    pub fn apply_curve(&mut self) {
        // Option<usize> is Copy, so this copies the curve out of self. The loop then borrows only self.scores.
        if let Some(curve) = self.curve {
            for score in self.scores.iter_mut() {
                *score += curve;
            }
        }
    }
}

fn main() {
    let mut result = TestResult {
        scores: vec![20, 50, 30],
        curve: Some(10),
    };
    result.apply_curve();
    println!("{:?}", result.scores);

    // Try: set curve to None and call apply_curve again. What do the scores become?
}
