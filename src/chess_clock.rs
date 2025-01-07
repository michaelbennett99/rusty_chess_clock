use std::{cmp::min, fmt::Display, time::Duration};
use crate::{Clock, ClockMode, ClockState, times};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    Player1,
    Player2,
}

impl Player {
    pub const ALL: [Self; 2] = [Self::Player1, Self::Player2];

    pub fn index(&self) -> usize {
        match self {
            Self::Player1 => 0,
            Self::Player2 => 1,
        }
    }

    pub fn other(&self) -> Self {
        match self {
            Self::Player1 => Self::Player2,
            Self::Player2 => Self::Player1,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::Player1 => "Player 1",
            Self::Player2 => "Player 2",
        };
        write!(f, "{}", label)
    }
}

/// Represents the status of the ongoing game
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Status {
    Stopped,
    Running,
    Finished,
}

/// Represents the timing method used for the chess clock
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TimingMethod {
    Fischer,
    Bronstein,
}

impl TimingMethod {
    pub const ALL: [Self; 2] = [Self::Fischer, Self::Bronstein];
}

impl Display for TimingMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::Fischer => "Fischer",
            Self::Bronstein => "Bronstein",
        };
        write!(f, "{}", label)
    }
}

/// Represents the rules for the chess clock: the time for each player,
/// the increment, the starter and the timing method
#[derive(Debug, Clone)]
pub struct Rules {
    player1_time: Duration,
    player2_time: Duration,
    increment: Duration,
    starter: Player,
    timing_method: TimingMethod,
}

impl Rules {
    /// Create a new custom set of rules for the chess clock
    pub fn new(
        player1_time: Duration, player2_time: Duration,
        increment: Duration, starter: Player, timing_method: TimingMethod
    ) -> Self {
        Self { player1_time, player2_time, increment, starter, timing_method }
    }

    /// The default rules for the chess clock
    pub fn default() -> Self {
        Self::new(
            times::TEN_MINUTES,
            times::TEN_MINUTES,
            times::FIVE_SECONDS,
            Player::Player1,
            TimingMethod::Fischer
        )
    }

    pub fn get_player1_time(&self) -> Duration {
        self.player1_time
    }

    pub fn get_player2_time(&self) -> Duration {
        self.player2_time
    }

    /// Get the time for the active player
    pub fn get_time(&self, state: Player) -> Duration {
        match state {
            Player::Player1 => self.player1_time,
            Player::Player2 => self.player2_time,
        }
    }

    pub fn get_increment(&self) -> Duration {
        self.increment
    }

    pub fn get_timing_method(&self) -> TimingMethod {
        self.timing_method
    }

    pub fn get_starter(&self) -> Player {
        self.starter
    }

    pub fn set_time(&mut self, state: Player, time: Duration) {
        match state {
            Player::Player1 => self.player1_time = time,
            Player::Player2 => self.player2_time = time,
        }
    }

    pub fn set_increment(&mut self, increment: Duration) {
        self.increment = increment;
    }

    pub fn set_timing_method(&mut self, timing_method: TimingMethod) {
        self.timing_method = timing_method;
    }

    pub fn set_starter(&mut self, starter: Player) {
        self.starter = starter;
    }
}

/// The chess clock.
#[derive(Debug)]
pub struct ChessClock {
    clocks: [Clock; 2],
    manually_finished: bool,
    active_player: Player,
    rules: Rules,
}

impl ChessClock {
    /// Create a new chess clock with the given rules
    pub fn new(rules: Rules) -> Self {
        Self {
            clocks: [
                Clock::new(
                    ClockMode::CountDown,
                    Some(rules.player1_time)
                ),
                Clock::new(
                    ClockMode::CountDown,
                    Some(rules.player2_time)
                ),
            ],
            manually_finished: false,
            active_player: rules.starter,
            rules,
        }
    }

    /// Create a new chess clock with the default ruleset
    pub fn default() -> Self {
        Self::new(Rules::default())
    }

    /// Get the active player
    pub fn active_player(&self) -> Player {
        self.active_player
    }

    /// Gets the time remaining for both players
    pub fn read(&self) -> (Duration, Duration) {
        (
            self.clocks[Player::Player1.index()].read(),
            self.clocks[Player::Player2.index()].read(),
        )
    }

    /// Get the status of the chess clock
    ///
    /// Status can be:
    /// - Stopped: The game is stopped
    /// - Running: The game is running
    /// - Finished: The game is finished. Happens if the game is manually
    ///   finished, or if the time runs out for one of the players.
    pub fn status(&self) -> Status {
        let (t1, t2) = self.read();
        let t = t1.as_secs_f64() * t2.as_secs_f64();
        let (s1, s2) = (
            self.clocks[Player::Player1.index()].state(),
            self.clocks[Player::Player2.index()].state()
        );

        match (t, s1, s2, self.manually_finished) {
            (_, _, _, true) => Status::Finished,
            (0.0, _, _, _) => Status::Finished,
            (_, ClockState::Stopped, ClockState::Stopped, _) => Status::Stopped,
            _ => Status::Running,
        }
    }

    /// Start the current player's clock
    fn start_current(&mut self) {
        self.clocks[self.active_player.index()].start();
    }

    /// Start the current player's clock
    pub fn start(&mut self) {
        self.start_current();
    }

    /// Switch the active player
    ///
    /// This can have an effect when the clock is running or stopped, but not
    /// when the game is finished. If the clock is stopped, the timing method
    /// will not be applied.
    pub fn switch_player(&mut self) {
        let current = self.active_player;
        let new = current.other();
        let current_status = self.status();

        if let Status::Running = current_status {
            // handle timing and stop current clock
            let running_time = self.clocks[current.index()]
                .read_running();
            self.clocks[current.index()].stop();

            // add increment to the current clock
            match self.rules.get_timing_method() {
                TimingMethod::Fischer => {
                    self.clocks[current.index()].add(self.rules.increment);
                }
                TimingMethod::Bronstein => {
                    self.clocks[current.index()].add(min(
                        running_time, self.rules.increment
                    ));
                }
            }

            // start the next clock
            self.clocks[new.index()].start();
            self.active_player = new;
        } else if let Status::Finished = current_status {
            // do nothing
        } else {
            self.active_player = new;
        }
    }

    /// Stop the current player's clock
    pub fn stop(&mut self) {
        self.clocks[self.active_player.index()].stop();
    }

    /// Manually finish the game
    pub fn finish(&mut self) {
        self.clocks.iter_mut().for_each(|clock| clock.stop());
        self.manually_finished = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Sleep;

    fn approx_eq(a: Duration, b: Duration) -> bool {
        let diff = if a > b { a - b } else { b - a };
        diff <= Duration::from_millis(10)
    }

    #[test]
    /// Test that the default chess clock is set up correctly
    fn test_chess_clock_default() {
        let clock = ChessClock::default();
        let (p1, p2) = clock.read();
        assert!(approx_eq(p1, times::TEN_MINUTES));
        assert!(approx_eq(p2, times::TEN_MINUTES));
        assert_eq!(clock.active_player(), Player::Player1);
        assert_eq!(clock.status(), Status::Stopped);
    }

    #[test]
    /// Test basic chess clock functionality with Fischer timing
    fn test_chess_clock_fischer() {
        let rules = Rules::new(
            Duration::from_secs(5),
            Duration::from_secs(5),
            Duration::from_secs(2),
            Player::Player1,
            TimingMethod::Fischer
        );
        let mut clock = ChessClock::new(rules);

        // Check initial state
        assert_eq!(clock.status(), Status::Stopped);

        // Start clock and check running
        clock.start();
        assert_eq!(clock.status(), Status::Running);
        Duration::from_secs(1).sleep();

        // Switch player and verify increment added
        clock.switch_player();
        let (p1_time, _) = clock.read();
        assert!(approx_eq(p1_time, Duration::from_secs(6))); // 5 - 1 + 2 increment
        assert_eq!(clock.active_player(), Player::Player2);

        // Let some time pass and switch back
        Duration::from_secs(2).sleep();
        clock.switch_player();
        let (_, p2_time) = clock.read();
        assert!(approx_eq(p2_time, Duration::from_secs(5))); // 5 - 2 + 2 increment
    }

    #[test]
    /// Test basic chess clock functionality with Bronstein timing
    fn test_chess_clock_bronstein() {
        let rules = Rules::new(
            Duration::from_secs(5),
            Duration::from_secs(5),
            Duration::from_secs(2),
            Player::Player1,
            TimingMethod::Bronstein
        );
        let mut clock = ChessClock::new(rules);

        clock.start();
        Duration::from_secs(1).sleep();

        // Switch player - should add 1 second (time used) not full 2 second increment
        clock.switch_player();
        let (p1_time, _) = clock.read();
        assert!(approx_eq(p1_time, Duration::from_secs(5))); // 5 - 1 + 1 used

        // Use more time than increment
        Duration::from_secs(3).sleep();
        clock.switch_player();
        let (_, p2_time) = clock.read();
        assert!(approx_eq(p2_time, Duration::from_secs(4))); // 5 - 3 + 2 max increment
    }

    #[test]
    /// Test that the clock finishes when time runs out
    fn test_chess_clock_finish() {
        let rules = Rules::new(
            Duration::from_secs(2),
            Duration::from_secs(2),
            Duration::ZERO,
            Player::Player1,
            TimingMethod::Fischer
        );
        let mut clock = ChessClock::new(rules);

        clock.start();
        assert_eq!(clock.status(), Status::Running);

        Duration::from_secs(3).sleep();
        assert_eq!(clock.status(), Status::Finished);
    }

    #[test]
    /// Test manual finish functionality
    fn test_chess_clock_manual_finish() {
        let mut clock = ChessClock::default();

        clock.start();
        assert_eq!(clock.status(), Status::Running);

        clock.finish();
        assert_eq!(clock.status(), Status::Finished);

        // Verify can't restart after finish
        clock.start();
        assert_eq!(clock.status(), Status::Finished);
    }
}
