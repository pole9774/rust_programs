use std::time::Instant;

pub struct Timer {
    label: String,
    start: Instant,
}

impl Timer {
    pub fn new(label: String) -> Self {
        Timer {
            label,
            start: Instant::now(),
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        println!(
            "Elapsed time for {}: {:?}",
            self.label,
            Instant::now() - self.start
        );
    }
}
