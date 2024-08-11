use std::{io::{self, Write}, thread::sleep, time::Duration};
use rusty_chess_clock::{Clock, ClockMode, ClockState, times::*};
use termion::{
    clear, input::{TermRead, Keys}, raw::IntoRawMode, AsyncReader
};

fn main() {
    println!("Clock");
    println!("=====");

    let mode = get_mode();
    let start = get_start_time();

    let mut clock = Clock::new(mode, start);
    run_clock(&mut clock);
}

fn run_clock(clock: &mut Clock) {
    let stdin = termion::async_stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut keys = stdin.keys();

    clock.start();
    while let ClockState::Running(_) = clock.state() {
        async_process_keys(clock, &mut keys);
        display_clock(clock, &mut stdout);
        sleep(Duration::from_millis(10));
    }
    println!("\rClock stopped at: {:#}", clock.read());
}

fn display_clock(
    clock: &mut Clock,
    stdout: &mut termion::raw::RawTerminal<io::Stdout>
) {
    print!("\r{}Clock: {:#}", clear::CurrentLine, clock.read());
    stdout.flush().unwrap();
}

fn get_mode() -> ClockMode {
    println!("Enter the mode of the clock:");
    println!("1. Count Up");
    println!("2. Count Down");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "1" => return ClockMode::CountUp,
            "2" => return ClockMode::CountDown,
            _ => println!("Invalid input. Please enter 1 or 2."),
        }
    }
}

fn get_start_time() -> Option<Duration> {
    println!("Enter the start time of the clock in seconds:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse() {
        Ok(seconds) => {
            Some(Duration::from_secs(seconds))
        },
        _ => {
            println!("Invalid start time, using defaults");
            None
        }
    }
}

fn async_process_keys(clock: &mut Clock, keys: &mut Keys<AsyncReader>) {
    if let Some(Ok(key)) = keys.next() {
        match key {
            termion::event::Key::Char(c) => match c {
                'q' => clock.stop(),
                'r' => {
                    clock.reset(None);
                    clock.start();
                },
                ']' => clock.add(ONE_SECOND),
                '[' => clock.subtract(ONE_SECOND),
                '\'' => clock.add(ONE_MINUTE),
                ';' => clock.subtract(ONE_MINUTE),
                '.' => clock.add(ONE_HOUR),
                ',' => clock.subtract(ONE_HOUR),
                _ => {}
            },
            _ => {}
        }
    }
}
