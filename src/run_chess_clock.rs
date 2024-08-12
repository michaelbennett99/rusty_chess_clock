use std::{io::{self, Write}, thread::sleep, time::Duration};
use rusty_chess_clock::{times::TEN_MINUTES, Rules, State, ChessClock, Status};
use termion::{clear, input::{TermRead, Keys}, raw::IntoRawMode, AsyncReader};

pub fn main() {
    let start_time = get_start_time();
    let increment = get_increment();
    let rules = Rules::new(
        start_time, start_time,
        increment, State::Player1
    );
    println!("========= Chess Clock =========");
    println!("Start time: {:?}", start_time);
    println!("Increment: {:?}", increment);
    println!("===================================");

    let mut chess_clock = ChessClock::new(rules);
    run_clock(&mut chess_clock);
}

fn get_start_time() -> Duration {
    print!("Time per player (default 10 minutes): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse::<u64>() {
        Ok(duration) => Duration::from_secs(duration * 60),
        Err(_) => TEN_MINUTES,
    }
}

fn get_increment() -> Duration {
    print!("Increment per move (default 0): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse::<u64>() {
        Ok(duration) => Duration::from_secs(duration),
        Err(_) => Duration::from_secs(0),
    }
}

fn run_clock(chess_clock: &mut ChessClock) {
    let stdin = termion::async_stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut keys = stdin.keys();

    while chess_clock.status() != Status::Finished {
        async_process_input(chess_clock, &mut keys);
        display_clock(chess_clock, &mut stdout);
        sleep(Duration::from_millis(10));
    }
    let _ = stdout.suspend_raw_mode();
    println!("");
    println!("Game finished!");
}

fn display_clock(
    chess_clock: &ChessClock,
    stdout: &mut termion::raw::RawTerminal<io::Stdout>
) {
    print!("\r{}{}", clear::CurrentLine, chess_clock);
    stdout.flush().unwrap();
}

fn async_process_input(
    chess_clock: &mut ChessClock,
    keys: &mut Keys<AsyncReader>
) {
    if let Some(Ok(key)) = keys.next() {
        match key {
            termion::event::Key::Char(c) => match c {
                'q' => {
                    if chess_clock.status() == Status::Running {
                        chess_clock.stop();
                    } else {
                        chess_clock.finish();
                    }
                },
                ' ' => {
                    chess_clock.switch_player();
                },
                '\n' => {
                    chess_clock.start();
                },
                _ => {}
            },
            _ => {}
        }
    }
}
