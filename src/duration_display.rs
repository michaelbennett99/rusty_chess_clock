use std::{fmt::Display, ops::Deref, time::Duration};
use crate::utils::round;

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
        let secs = secs as f64 + millis as f64 / 1000.0;
        let (precision, width) = if f.alternate() {
            (2, 5)
        } else {
            (0, 2)
        };
        let time = round(secs, precision);
        let mins = time as u32 / 60;
        let secs = time % 60.0;
        if mins >= 60 {
            let hours = mins / 60;
            let mins = mins % 60;
            return write!(
                f, "{:02}:{:02}:{:03$.4$}",
                hours, mins, secs, width, precision as usize
            );
        }
        write!(
            f, "{:02}:{:02$.3$}", mins, secs, width, precision as usize
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_65() {
        let duration = Duration::from_secs(65);
        let display = DurationDisplay::from(duration);
        assert_eq!(display.to_string(), "01:05");

        assert_eq!(format!("{:#}", display), "01:05.00");
    }

    #[test]
    fn test_display_59_99() {
        let duration = Duration::from_secs(59)
            .checked_add(Duration::from_millis(990))
            .unwrap();
        let display = DurationDisplay::from(duration);
        assert_eq!(display.to_string(), "01:00");

        assert_eq!(format!("{:#}", display), "00:59.99");
    }

    #[test]
    fn test_display_0() {
        let duration = Duration::from_secs(0);
        let display = DurationDisplay::from(duration);
        assert_eq!(display.to_string(), "00:00");

        assert_eq!(format!("{:#}", display), "00:00.00");
    }

    #[test]
    fn test_display_60() {
        let duration = Duration::from_secs(60);
        let display = DurationDisplay::from(duration);
        assert_eq!(display.to_string(), "01:00");

        assert_eq!(format!("{:#}", display), "01:00.00");
    }

    #[test]
    fn test_display_600() {
        let duration = Duration::from_secs(600);
        let display = DurationDisplay::from(duration);
        assert_eq!(display.to_string(), "10:00");

        assert_eq!(format!("{:#}", display), "10:00.00");
    }

    #[test]
    fn test_display_600_1() {
        let duration = Duration::from_secs(600)
            .checked_add(Duration::from_millis(100))
            .unwrap();
        let display = DurationDisplay::from(duration);
        assert_eq!(display.to_string(), "10:00");

        assert_eq!(format!("{:#}", display), "10:00.10");
    }

    #[test]
    fn test_display_3600() {
        let duration = Duration::from_secs(3600);
        let display = DurationDisplay::from(duration);
        assert_eq!(display.to_string(), "01:00:00");

        assert_eq!(format!("{:#}", display), "01:00:00.00");
    }
}
