use limit_tracker::{LimitTracker, Messenger};

// A messenger that shows each message on screen. An application could send email or text messages instead.
struct PrintMessenger;

impl Messenger for PrintMessenger {
    fn send(&self, msg: &str) {
        println!("{msg}");
    }
}

fn main() {
    let messenger = PrintMessenger;
    let mut tracker = LimitTracker::new(&messenger, 100);

    tracker.set_value(50);
    tracker.set_value(80);
    tracker.set_value(95);
    tracker.set_value(101);

    // Try: change 95 to 90. Which warning appears for 90, and why?
}
