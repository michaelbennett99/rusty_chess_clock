use std::{
    io, time::Duration
};
use rusty_chess_clock::{Clock, ClockMode, Sleep};

fn main() {
    println!("Clock");
    println!("=====");

    let mode = get_mode();
    let start = get_start_time();

    let mut clock = Clock::new(mode, start);
    clock.start();
    loop {
        println!("Clock: {}", clock.read());
        1000u64.sleep();
    }
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
    let start: u64 = start.trim().parse().expect("Please enter a number");
    if start > 0 {
        Some(Duration::from_secs(start))
    } else {
        None
    }
}
