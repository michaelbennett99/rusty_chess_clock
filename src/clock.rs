use crate::DurationDisplay;
use std::{fmt::Display, time::{Duration, Instant}};

const TEN_MINUTES: Duration = Duration::from_secs(60 * 10);

/// ClockState records whether the clock is running or stopped, and the time at
/// which it was last started if it is running.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClockState {
    Running(Instant),
    Stopped,
    Finished,
}

/// ClockMode records whether the clock should count up or down.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClockMode {
    CountUp,
    CountDown,
}

/// A simple clock that can be started and stopped
///
/// The clock starts at 0 and can be read at any time, it can be started,
/// stopped and reset.
/// Works by keeping track of the last start time and the total time that has
/// passed between the last reset and the last stop.
///
/// When counting down, the clock automatically stops when it reaches zero.
pub struct Clock {
    already_elapsed: Duration,
    state: ClockState,
    mode: ClockMode
}

impl Clock {
    /// Constructs a new stopped clock with the given time
    pub fn new(mode: ClockMode, start: Option<Duration>) -> Clock {
        let elapsed = match (mode, start) {
            (_, Some(start)) => start,
            (ClockMode::CountUp, None) => Duration::ZERO,
            (ClockMode::CountDown, None) => TEN_MINUTES,
        };

        Clock {
            already_elapsed: elapsed,
            state: ClockState::Stopped,
            mode
        }
    }

    /// Initialise a new clock that counts up from 0 with no start time
    pub fn default() -> Self {
        Self::new(ClockMode::CountUp, None)
    }

    /// Read the current time on the clock
    ///
    /// This is a read-only function, and it will not update the state of the
    /// clock.
    pub fn read(&self) -> Duration {
        match (&self.state, &self.mode) {
            (ClockState::Running(start), ClockMode::CountUp) => {
                let now = Instant::now();
                let elapsed = now - *start;
                self.already_elapsed + elapsed
            },
            (ClockState::Running(start), ClockMode::CountDown) => {
                let now = Instant::now();
                let elapsed = now - *start;
                self.already_elapsed.saturating_sub(elapsed)
            }
            (_, _) => self.already_elapsed,
        }
    }

    /// Read the amount of time that has passed since the clock was last started
    ///
    /// This is a read-only function, and it will not update the state of the
    /// clock.
    pub fn read_running(&self) -> Duration {
        match self.state {
            ClockState::Running(start) => Instant::now() - start,
            _ => Duration::ZERO,
        }
    }

    /// Read the current time on the clock and update the state of the clock
    /// if necessary
    ///
    /// If the clock is in CountDown mode and the time is zero, the clock will
    /// be stopped.
    pub fn read_and_update(&mut self) -> Duration {
        let time = self.read();

        let is_running = matches!(self.state, ClockState::Running(_));
        let is_countdown = self.mode == ClockMode::CountDown;
        let is_zero = time == Duration::ZERO;

        if is_running && is_countdown && is_zero {
            self.already_elapsed = Duration::ZERO;
            self.state = ClockState::Stopped;
        }

        time
    }

    /// Get the current (possibly deprecated) state of the clock
    ///
    /// State could be deprecated if it hasn't been updated and a countdown
    /// clock has reached zero and stopped. In this case, the state would
    /// still show as running.
    pub fn state(&self) -> ClockState {
        self.state
    }

    /// Get the current state of the clock
    ///
    /// State is guaranteed to be updated by this function.
    pub fn state_and_update(&mut self) -> ClockState {
        self.read_and_update();
        self.state
    }

    /// Starts the clock
    ///
    /// If the clock is already running, this does nothing
    pub fn start(&mut self) {
        if let ClockState::Stopped = self.state {
            self.state = ClockState::Running(Instant::now());
        }
    }

    /// Stops the clock.
    ///
    /// If the clock is already stopped, this does nothing.
    pub fn stop(&mut self) {
        // If the clock is running, read the current time and set the elapsed
        // time to the current time
        if let ClockState::Running(_) = self.state {
            self.already_elapsed = self.read();
            self.state = ClockState::Stopped;
        }
    }

    /// Resets the clock
    ///
    /// Sets the elapsed time to start (or zero) and stops the clock
    pub fn reset(&mut self, start: Option<Duration>) {
        self.already_elapsed = start.unwrap_or(Duration::ZERO);
        self.state = ClockState::Stopped;
    }

    /// Resets the clock to zero
    ///
    /// Sets the elapsed time to zero and stops the clock
    pub fn zero(&mut self) {
        self.reset(Some(Duration::ZERO));
    }

    /// Adds time to the clock
    pub fn add(&mut self, time: Duration) {
        self.already_elapsed = self.already_elapsed.saturating_add(time);
    }

    /// Subtracts time from the clock
    ///
    /// If the time to subtract is greater than the current time on the clock,
    /// the clock will be set to zero.
    pub fn subtract(&mut self, time: Duration) {
        if let ClockState::Running(_) = self.state {
            let total_time = self.read();
            self.reset(Some(total_time.saturating_sub(time)));
            if self.already_elapsed > Duration::ZERO {
                self.start();
            }
        } else {
            self.already_elapsed = self.already_elapsed.saturating_sub(time);
        }
    }

    pub fn finish(&mut self) {
        self.stop();
        self.state = ClockState::Finished;
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let duration = DurationDisplay::from(self.read());
        if f.alternate() {
            write!(f, "{:#}", duration)
        } else {
            write!(f, "{}", duration)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Sleep;

    #[test]
    /// Test that the default clock is at 0, stopped and counting up
    fn test_clock_default() {
        let clock = Clock::default();
        assert_eq!(clock.to_string(), "00:00");
        assert_eq!(clock.state(), ClockState::Stopped);
        assert_eq!(clock.mode, ClockMode::CountUp);
    }

    #[test]
    /// Test that the clock behaves as expected when counting up
    fn test_clock_count_up() {
        let mut clock = Clock::new(ClockMode::CountUp, None);
        assert_eq!(clock.to_string(), "00:00");

        clock.start();
        Duration::from_secs(1).sleep();
        assert_eq!(clock.to_string(), "00:01");

        clock.stop();
        Duration::from_secs(1).sleep();
        assert_eq!(clock.to_string(), "00:01");

        clock.start();
        Duration::from_secs(1).sleep();
        assert_eq!(clock.to_string(), "00:02");

        clock.reset(Some(Duration::from_secs(5)));
        assert_eq!(clock.to_string(), "00:05");

        clock.zero();
        assert_eq!(clock.to_string(), "00:00");
    }

    #[test]
    fn test_clock_count_down() {
        let mut clock = Clock::new(
            ClockMode::CountDown, None
        );
        assert_eq!(clock.to_string(), "10:00");

        clock.start();
        Duration::from_secs(1).sleep();
        assert_eq!(clock.to_string(), "09:59");

        clock.stop();
        Duration::from_secs(1).sleep();
        assert_eq!(clock.to_string(), "09:59");

        clock.start();
        Duration::from_secs(1).sleep();
        assert_eq!(clock.to_string(), "09:58");

        clock.reset(Some(Duration::from_secs(5)));
        assert_eq!(clock.to_string(), "00:05");

        clock.zero();
        assert_eq!(clock.to_string(), "00:00");

        clock.reset(Some(Duration::from_millis(750)));
        assert_eq!(clock.to_string(), "00:01");
        assert!(matches!(clock.state(), ClockState::Stopped));

        clock.start();
        assert!(matches!(clock.state(), ClockState::Running(_)));

        Duration::from_secs(1).sleep();
        clock.read_and_update();
        assert_eq!(clock.state(), ClockState::Stopped);
        assert_eq!(clock.to_string(), "00:00");
    }

    #[test]
    fn test_clock_add_subtract() {
        let mut clock = Clock::new(ClockMode::CountUp, None);
        assert_eq!(clock.to_string(), "00:00");

        clock.add(Duration::from_secs(5));
        assert_eq!(clock.to_string(), "00:05");

        clock.subtract(Duration::from_secs(2));
        assert_eq!(clock.to_string(), "00:03");

        clock.subtract(Duration::from_secs(5));
        assert_eq!(clock.to_string(), "00:00");
    }

    #[test]
    fn test_running_clock_add_subtract() {
        let mut clock = Clock::new(ClockMode::CountUp, None);
        assert_eq!(clock.to_string(), "00:00");

        clock.start();
        Duration::from_secs(5).sleep();
        assert_eq!(clock.to_string(), "00:05");

        clock.add(Duration::from_secs(5));
        assert_eq!(clock.to_string(), "00:10");

        clock.subtract(Duration::from_secs(2));
        assert_eq!(clock.to_string(), "00:08");

        clock.subtract(Duration::from_secs(10));
        assert_eq!(clock.to_string(), "00:00");
        assert_eq!(clock.state(), ClockState::Stopped);
    }
}
