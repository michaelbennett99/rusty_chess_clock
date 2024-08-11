//! # Sleep
//! Submodule for simple spinning sleep functions required to precisely test the
//! clock module.

use std::time::{Duration, Instant};

/// Sleep trait requires that implementors can sleep for a given duration.
pub trait Sleep {
    fn sleep(&self);
}


impl Sleep for Duration {
    /// Sleep for the given duration
    fn sleep(&self) {
        let now = Instant::now();
        let end = now + *self;
        while Instant::now() < end {}
    }
}

impl Sleep for u64 {
    /// Sleep for `self` milliseconds
    fn sleep(&self) {
        Duration::from_millis(*self).sleep();
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_sleep() {
        let now = Instant::now();

        let duration = Duration::from_millis(10);
        duration.sleep();

        let elapsed = now.elapsed();
        // check that elapsed time is close to 10 milliseconds
        assert_eq!(elapsed.as_millis(), 10);
    }
}
