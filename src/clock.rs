use crate::duration_display::DurationDisplay;
use std::time::{Duration, Instant};

const TEN_MINUTES: Duration = Duration::from_secs(60 * 10);

/// ClockState records whether the clock is running or stopped, and the time at
/// which it was last started if it is running.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClockState {
    Running(Instant),
    Stopped,
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
    /// Current design requires a mutable reference to self to update the state
    /// of the clock if the clock is zero while in CountDown mode. Unsure if
    /// this is the best design, but it makes sense to stop the clock if it
    /// reaches zero.
    fn _read(&mut self) -> Duration {
        match (&self.state, &self.mode) {
            (ClockState::Running(start), ClockMode::CountUp) => {
                let now = Instant::now();
                let elapsed = now - *start;
                self.already_elapsed + elapsed
            }
            (ClockState::Running(start), ClockMode::CountDown) => {
                let now = Instant::now();
                let elapsed = now - *start;
                let time = self
                    .already_elapsed.saturating_sub(elapsed);
                if time <= Duration::ZERO {
                    // If the time is less than or equal to zero, stop the clock
                    // Cannot use self.stop() here because it uses self._read()
                    // which would cause an infinite loop
                    self.already_elapsed = Duration::ZERO;
                    self.state = ClockState::Stopped;
                }
                time
            }
            (ClockState::Stopped, _) => self.already_elapsed,
        }
    }

    /// Read the current time on the clock
    /// Must take a mutable reference to self as ._read() requires it
    pub fn read(&mut self) -> DurationDisplay {
        // handle getting the current time based on the state of the clock
        self._read().into()
    }

    /// Get the current state of the clock
    pub fn state(&self) -> ClockState {
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
            self.already_elapsed = self._read();
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
}
