use std::{fmt::Display, time::Duration};
use crate::{Clock, ClockMode, ClockState, DurationDisplay};

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

#[derive(Debug, PartialEq)]
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

    pub fn status(&mut self) -> Status {
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

impl Display for ChessClock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (p1, p2) = self.read();
        write!(
            f, "Player 1: {}    Player 2: {}",
            DurationDisplay::from(p1),
            DurationDisplay::from(p2)
        )
    }
}
