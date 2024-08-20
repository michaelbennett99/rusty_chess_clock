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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Status {
    Stopped,
    Running,
    Finished,
}

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

#[derive(Debug, Clone)]
pub struct Rules {
    player1_time: Duration,
    player2_time: Duration,
    increment: Duration,
    starter: Player,
    timing_method: TimingMethod,
}

impl Rules {
    pub fn new(
        player1_time: Duration, player2_time: Duration,
        increment: Duration, starter: Player, timing_method: TimingMethod
    ) -> Self {
        Self { player1_time, player2_time, increment, starter, timing_method }
    }

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

#[derive(Debug)]
pub struct ChessClock {
    clocks: [Clock; 2],
    state: Player,
    rules: Rules,
}

impl ChessClock {
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
            state: rules.starter,
            rules,
        }
    }

    pub fn default() -> Self {
        Self::new(Rules::new(
            times::TEN_MINUTES,
            times::TEN_MINUTES,
            times::FIVE_SECONDS,
            Player::Player1,
            TimingMethod::Fischer
        ))
    }

    pub fn active_player(&self) -> Player {
        self.state
    }

    pub fn read(&self) -> (Duration, Duration) {
        (
            self.clocks[Player::Player1.index()].read(),
            self.clocks[Player::Player2.index()].read(),
        )
    }

    pub fn update(&mut self) {
        self.clocks.iter_mut()
            .for_each(|clock| { clock.read_and_update(); });
    }

    pub fn status(&self) -> Status {
        let (t1, t2) = self.read();
        let (s1, s2) = (
            self.clocks[Player::Player1.index()].state(),
            self.clocks[Player::Player2.index()].state()
        );

        match (t1.as_secs_f64() * t2.as_secs_f64(), s1, s2) {
            (0.0, _, _) => Status::Finished,
            (_, ClockState::Finished, ClockState::Finished) => Status::Finished,
            (_, ClockState::Stopped, ClockState::Stopped) => Status::Stopped,
            _ => Status::Running,
        }
    }

    fn start_current(&mut self) {
        self.clocks[self.state.index()].start();
    }

    pub fn start(&mut self) {
        self.start_current();
    }

    pub fn switch_player(&mut self) {
        self.update();

        let current = self.state;
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
            self.state = new;
        } else if let Status::Finished = current_status {
            // do nothing
        } else {
            self.state = new;
        }
    }

    pub fn stop(&mut self) {
        self.clocks[self.state.index()].stop();
    }

    pub fn finish(&mut self) {
        self.clocks.iter_mut().for_each(|clock| clock.finish());
    }
}
