use std::{fmt::Display, time::Duration};
use crate::{Clock, ClockMode, ClockState, DurationDisplay};
use termion::color::{self, Color};

pub struct Rules {
    player1_time: Duration,
    player2_time: Duration,
    increment: Duration,
    starter: State,
}

impl Rules {
    pub fn new(
        player1_time: Duration, player2_time: Duration,
        increment: Duration, starter: State
    ) -> Self {
        Self { player1_time, player2_time, increment, starter }
    }

    pub fn get_player1_time(&self) -> Duration {
        self.player1_time
    }

    pub fn get_player2_time(&self) -> Duration {
        self.player2_time
    }

    pub fn get_increment(&self) -> Duration {
        self.increment
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    Player1,
    Player2,
}

impl State {
    pub fn index(self) -> usize {
        match self {
            Self::Player1 => 0,
            Self::Player2 => 1,
        }
    }

    pub fn other(self) -> Self {
        match self {
            Self::Player1 => Self::Player2,
            Self::Player2 => Self::Player1,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Status {
    Stopped,
    Running,
    Finished,
}

pub struct ChessClock {
    clocks: [Clock; 2],
    state: State,
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

    pub fn read(&self) -> (Duration, Duration) {
        (
            self.clocks[State::Player1.index()].read(),
            self.clocks[State::Player2.index()].read(),
        )
    }

    pub fn update(&mut self) {
        self.clocks.iter_mut()
            .for_each(|clock| { clock.read_and_update(); });
    }

    pub fn status(&self) -> Status {
        let (t1, t2) = self.read();
        let (s1, s2) = (
            self.clocks[State::Player1.index()].state(),
            self.clocks[State::Player2.index()].state()
        );

        match (t1.as_secs() * t2.as_secs(), s1, s2) {
            (0, _, _) => Status::Finished,
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
        let current = self.state;
        let new = current.other();

        if let ClockState::Running(_) = self.clocks[current.index()].state() {
            self.clocks[current.index()].stop();
            self.clocks[current.index()].add(self.rules.increment);
            self.clocks[new.index()].start();
        }
        self.state = new;
    }

    pub fn stop(&mut self) {
        self.clocks[self.state.index()].stop();
    }

    pub fn finish(&mut self) {
        self.clocks.iter_mut().for_each(|clock| clock.finish());
    }
}

impl Color for Status {
    fn write_fg(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Running => color::Green.write_fg(f),
            Self::Stopped => color::Yellow.write_fg(f),
            Self::Finished => color::Red.write_fg(f),
        }
    }

    fn write_bg(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Running => color::Green.write_bg(f),
            Self::Stopped => color::Yellow.write_bg(f),
            Self::Finished => color::Red.write_bg(f),
        }
    }
}

impl Display for ChessClock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (p1, p2) = self.read();

        let fg = color::White;
        let bg = self.status();

        /// Display a player's time and label with context-award colouring
        macro_rules! display_player {
            ($player:expr, $time:expr, $label:expr) => {
                if self.state == $player {
                    write!(f, "{}{}", color::Fg(fg), color::Bg(bg))?;
                }
                write!(f, " {}: {} ", $label, DurationDisplay::from($time))?;
                if self.state == $player {
                    write!(f, "{}{}", color::Fg(color::Reset), color::Bg(color::Reset))?;
                }
            };
        }

        display_player!(State::Player1, p1, "Player 1");
        display_player!(State::Player2, p2, "Player 2");

        Ok(())
    }
}
