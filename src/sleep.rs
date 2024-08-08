// Implement a custom sleep

use std::time::{Duration, Instant};

pub trait Sleep {
    fn sleep(&self);
}

impl Sleep for Duration {
    fn sleep(&self) {
        let now = Instant::now();
        let end = now + *self;
        while Instant::now() < end {}
    }
}

impl Sleep for u64 {
    fn sleep(&self) {
        Duration::from_millis(*self).sleep();
    }
}
