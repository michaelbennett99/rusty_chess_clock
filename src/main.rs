use std::{
    io::{self, Write}, thread::sleep, time::Duration
};
use rusty_chess_clock::{Clock, ClockMode, ClockState};
use termion::{
    input::TermRead, raw::IntoRawMode
};

fn main() {
    println!("Clock");
    println!("=====");

    let mode = get_mode();
    let start = get_start_time();

    let stdin = termion::async_stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut keys = stdin.keys();

    let mut clock = Clock::new(mode, start);
    clock.start();
    while let ClockState::Running(_) = clock.state() {
        // input logic
        if let Some(Ok(key)) = keys.next() {
            match key {
                termion::event::Key::Char('q') => clock.stop(),
                termion::event::Key::Char('s') => clock.stop(),
                _ => {}
            }
        }

        print!("\rClock: {:#}", clock.read());
        stdout.flush().unwrap();
        sleep(Duration::from_millis(10));
    };
    println!("\rClock stopped at: {:#}", clock.read());
}

fn get_mode() -> ClockMode {
    println!("Enter the mode of the clock (1 for CountUp, 2 for CountDown):");
    let mut mode = String::new();
    io::stdin().read_line(&mut mode).expect("Failed to read line");
    let mode = mode.trim().parse().expect("Please enter a number");
    match mode {
        1 => ClockMode::CountUp,
        2 => ClockMode::CountDown,
        _ => {
            println!("Invalid mode, defaulting to CountUp");
            ClockMode::CountUp
        }
    }
}

fn get_start_time() -> Option<Duration> {
    println!("Enter the start time of the clock in seconds:");
    let mut start = String::new();
    io::stdin().read_line(&mut start).expect("Failed to read line");
    let start = match start.trim().parse() {
        Ok(num) if num > 0 => Some(num),
        _ => {
            println!("Invalid start time, using defaults");
            None
        }
    };

    start.map(|secs| Duration::from_secs(secs))
}
