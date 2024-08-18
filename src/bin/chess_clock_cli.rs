use std::{io::{self, Write}, thread::sleep, time::Duration};
use rusty_chess_clock::{
    times, Rules, State, ChessClock, Status, DurationDisplay, TimingMethod
};
use termion::{
    color::{self, Color},
    clear,
    input::{TermRead, Keys},
    raw::IntoRawMode,
    AsyncReader
};

fn main() {
    let start_time = get_start_time();
    let increment = get_increment();
    let timing_method = get_timing_method();
    let rules = Rules::new(
        start_time, start_time,
        increment, State::Player1, timing_method
    );
    print_instructions(&rules);

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
        Err(_) => times::TEN_MINUTES,
    }
}

fn get_increment() -> Duration {
    print!("Increment per move (default 5 seconds): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse::<u64>() {
        Ok(duration) => Duration::from_secs(duration),
        Err(_) => times::FIVE_SECONDS,
    }
}

fn get_timing_method() -> TimingMethod {
    loop {
        print!(
            "\r{}Timing method (f for Fischer, b for Bronstein): ",
            clear::CurrentLine
        );
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(_) => continue,
        }

        match input.trim() {
            "f" => return TimingMethod::Fischer,
            "b" => return TimingMethod::Bronstein,
            _ => continue,
        }
    }
}

fn print_instructions(rules: &Rules) {
    println!("===================== Chess Clock ====================");
    println!(
        "Player 1 time: {}, Player 2 time: {}",
        DurationDisplay::from(rules.get_player1_time()),
        DurationDisplay::from(rules.get_player2_time())
    );
    println!("Extra time: {}", DurationDisplay::from(rules.get_increment()));
    println!("Timing Method: {:?}", rules.get_timing_method());
    println!("Instructions:");
    println!("- Active player is indicated by highlighted background");
    println!("- Yellow: Stopped, Green: Running, Red: Finished");
    println!("- Press enter to start/stop");
    println!("- Press space to switch player");
    println!("- Press q to quit");
    println!("======================================================");
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
    print!("\r{}{}", clear::CurrentLine, format_chess_clock(chess_clock));
    stdout.flush().unwrap();
}

#[derive(Debug, Clone, Copy)]
struct StatusColor(Status);

impl Color for StatusColor {
    fn write_fg(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.0 {
            Status::Running => color::Green.write_fg(f),
            Status::Stopped => color::Yellow.write_fg(f),
            Status::Finished => color::Red.write_fg(f),
        }
    }

    fn write_bg(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.0 {
            Status::Running => color::Green.write_bg(f),
            Status::Stopped => color::Yellow.write_bg(f),
            Status::Finished => color::Red.write_bg(f),
        }
    }
}

pub fn format_chess_clock(clock: &ChessClock) -> String {
    let (p1, p2) = clock.read();

    let fg = color::White;
    let bg = StatusColor(clock.status());

    let mut result = String::new();

    macro_rules! display_player {
        ($player:expr, $time:expr, $label:expr) => {
            if clock.active_player() == $player {
                result.push_str(&format!("{}{}", color::Fg(fg), color::Bg(bg)));
            }
            result.push_str(&format!(
                " {}: {} ", $label, DurationDisplay::from($time)
            ));
            if clock.active_player() == $player {
                result.push_str(&format!(
                    "{}{}",
                    color::Fg(color::Reset),
                    color::Bg(color::Reset)
                ));
            }
        };
    }

    display_player!(State::Player1, p1, "Player 1");
    display_player!(State::Player2, p2, "Player 2");

    result
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
                    }
                    chess_clock.finish();
                },
                ' ' => {
                    chess_clock.switch_player();
                },
                '\n' => {
                    if chess_clock.status() == Status::Running {
                        chess_clock.stop();
                    } else {
                        chess_clock.start();
                    }
                },
                _ => {}
            },
            _ => {}
        }
    }
}
