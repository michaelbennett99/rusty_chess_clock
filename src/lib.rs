use std::{ops::Deref, fmt::Display, time::{Duration, Instant}};

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


#[derive(Debug, PartialEq)]
enum ClockState {
    Running(Instant),
    Stopped,
}

/*
A simple clock that can be started and stopped

The clock starts at 0 and can be read at any time, it can be started, stopped
and reset.
Works by keeping track of the last start time and the total time that has
passed between the last reset and the last stop.
*/
pub struct Clock {
    already_elapsed: Duration,
    state: ClockState,
}

impl Clock {
    // Constructs a new stopped clock with the given time
    pub fn new() -> Clock {
        Clock {
            already_elapsed: Duration::ZERO,
            state: ClockState::Stopped,
        }
    }

    // Read the current time on the clock
    pub fn read(&self) -> DurationDisplay {
        match self.state {
            ClockState::Running(start) => {
                let now = Instant::now();
                let elapsed = now - start;
                (elapsed + self.already_elapsed).into()
            }
            ClockState::Stopped => self.already_elapsed.into(),
        }
    }

    // Starts the clock
    // If the clock is already running, this does nothing
    pub fn start(&mut self) {
        if self.state == ClockState::Stopped {
            self.state = ClockState::Running(Instant::now());
        }
    }

    pub fn read_and_start(&mut self) -> DurationDisplay {
        let time = self.read();
        self.start();
        time
    }

    // Stops the clock
    // Returns the elapsed time since the clock was last reset.
    // If the clock is already stopped, this does nothing and returns the
    // elapsed time.
    pub fn stop(&mut self) {
        // If the clock is running, record elapsed time since start, add to
        // already_elapsed and set state to stopped
        match self.state {
            ClockState::Running(start) => {
                let now = Instant::now();
                let elapsed = now - start;
                self.already_elapsed += elapsed;
                self.state = ClockState::Stopped;
            }
            ClockState::Stopped => {}
        }
    }

    pub fn stop_and_read(&mut self) -> DurationDisplay {
        self.stop();
        self.read()
    }

    pub fn reset(&mut self) {
        self.already_elapsed = Duration::ZERO;
        self.state = ClockState::Stopped;
    }
}
