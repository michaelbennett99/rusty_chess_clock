use std::{fmt::Display, ops::Deref, time::Duration};

// Give wrapper class for Duration to implement Display, automatically convert
// Duration to DurationDisplay
#[derive(Debug, PartialEq)]
pub struct DurationDisplay(Duration);

impl From<Duration> for DurationDisplay {
    fn from(duration: Duration) -> Self {
        DurationDisplay(duration)
    }
}

impl Deref for DurationDisplay {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for DurationDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let secs = self.0.as_secs();
        let millis = self.0.subsec_millis();
        write!(f, "{}.{:02}s", secs, millis)
    }
}
