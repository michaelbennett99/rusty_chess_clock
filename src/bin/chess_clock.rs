// Use iced to create a GUI for the chess clock
use rusty_chess_clock::{
    ChessClock, DurationDisplay, State, Status, Rules, TimingMethod
};
use iced::{
    alignment,
    executor,
    keyboard,
    theme::{self, Theme},
    time,
    widget::{button, column, container, pick_list, row, text, text_input},
    Application,
    Color,
    Command,
    Element,
    Length,
    Renderer,
    Sandbox,
    Settings,
    Subscription
};
use std::time::Duration;

mod constants {
    pub const WIDTH: u16 = 400;
    pub const HEIGHT: u16 = 400;
    pub const SPACING: u16 = 20;
    pub const CLOCK_TEXT_SIZE: u16 = 50;
    pub const TEXT_SIZE: u16 = 30;
    pub const HEADER_SIZE: u16 = 50;
}

fn main() -> iced::Result {
    let _rules = Rules::default();
    <ChessClockSettings as Sandbox>::run(Settings::default())
}

struct ChessClockSettings {
    rules: Rules,
    time_strings: [String; 2],
    increment_string: String,
}

#[derive(Debug, Clone)]
enum SettingsMessage {
    ChangeTime(State, String),
    ChangeIncrement(String),
    ChangeTimingMethod(TimingMethod),
    ChangeActivePlayer(State),
    StartClock(Rules),
}

impl ChessClockSettings {
    fn time_selector(&self, player: State) -> Element<SettingsMessage> {
        let index = player.index();
        let time_str = &self.time_strings[index];
        let placeholder = format!("Enter {} time", player);

        let time_input = text_input(&placeholder, time_str)
            .on_input(move |input| {
                SettingsMessage::ChangeTime(
                    player, input
                )
            })
            .padding(constants::SPACING);


        let time_label = text(format!("{} time", player).as_str())
            .vertical_alignment(alignment::Vertical::Center);

        let time_printout = text(
            DurationDisplay::from(self.rules.get_time(player)).to_string()
        )
            .vertical_alignment(alignment::Vertical::Center);

        row![time_label, time_input, time_printout]
            .spacing(constants::SPACING)
            .width(Length::Fill)
            .into()
    }

    fn increment_selector(&self) -> Element<SettingsMessage> {
        let increment = &self.increment_string;
        let placeholder = "Enter increment";

        let increment_input = text_input(placeholder, increment)
            .on_input(SettingsMessage::ChangeIncrement);

        let increment_label = text("Increment")
            .vertical_alignment(alignment::Vertical::Center);

        let increment_printout = text(
            DurationDisplay::from(self.rules.get_increment()).to_string()
        )
            .vertical_alignment(alignment::Vertical::Center);

        row![increment_label, increment_input, increment_printout]
            .spacing(constants::SPACING)
            .width(Length::Fill)
            .into()
    }

    fn timing_method_selector(&self) -> Element<SettingsMessage> {
        let pick_list = pick_list(
            &TimingMethod::ALL[..],
            Some(self.rules.get_timing_method()),
            SettingsMessage::ChangeTimingMethod
        );

        let label = text("Timing method");
        let printout = text(self.rules.get_timing_method().to_string());
        row![label, pick_list, printout].spacing(constants::SPACING).into()
    }

    fn active_player_selector(&self) -> Element<SettingsMessage> {
        let pick_list = pick_list(
            &State::ALL[..],
            Some(self.rules.get_starter()),
            SettingsMessage::ChangeActivePlayer
        );

        let label = text("Active player");
        let printout = text(self.rules.get_starter().to_string());
        row![label, pick_list, printout].spacing(constants::SPACING).into()
    }

    fn start_button(&self) -> Element<SettingsMessage> {
        let action = SettingsMessage::StartClock(self.rules.clone());

        let button = button(text("Start clock"))
            .on_press(action)
            .style(theme::Button::Primary)
            .padding(constants::SPACING);

        button.into()
    }
}

impl Sandbox for ChessClockSettings {
    type Message = SettingsMessage;

    fn new() -> Self {
        let rules = Rules::default();
        Self {
            rules,
            time_strings: ["".to_string(), "".to_string()],
            increment_string: "".to_string(),
        }
    }

    fn title(&self) -> String {
        String::from("Chess Clock - Select Settings")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            SettingsMessage::ChangeTime(state, time) => {
                self.time_strings[state.index()] = time.clone();
                match time.parse::<u64>() {
                    Ok(minutes) => {
                        self.rules.set_time(
                            state, Duration::from_secs(minutes*60)
                        );
                    }
                    Err(_) => {
                        self.rules.set_time(state, Duration::from_secs(0));
                    }
                }
            }
            SettingsMessage::ChangeIncrement(increment) => {
                self.increment_string = increment.clone();
                match increment.parse::<u64>() {
                    Ok(seconds) => {
                        self.rules.set_increment(Duration::from_secs(seconds));
                    }
                    Err(_) => {
                        self.rules.set_increment(Duration::from_secs(0));
                    }
                }
            }
            SettingsMessage::ChangeTimingMethod(timing_method) => {
                self.rules.set_timing_method(timing_method);
            }
            SettingsMessage::ChangeActivePlayer(starter) => {
                self.rules.set_starter(starter);
            }
            SettingsMessage::StartClock(_rules) => {
                // ChessClockView::run(Settings::with_flags(rules));
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let header_text = text("Chess Clock")
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center)
            .size(constants::HEADER_SIZE)
            .style(theme::Text::Color(Color::BLACK))
            .width(Length::Fill);

        let p1_time_row = self.time_selector(State::Player1);
        let p2_time_row = self.time_selector(State::Player2);
        let increment_row = self.increment_selector();
        let timing_method_row = self.timing_method_selector();
        let active_player_row = self.active_player_selector();
        let start_button = self.start_button();

        container(column![
            header_text,
            p1_time_row,
            p2_time_row,
            increment_row,
            timing_method_row,
            active_player_row,
            start_button
        ].spacing(constants::SPACING))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(alignment::Vertical::Center)
            .center_x()
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone)]
enum Message {
    Update,
    SwitchPlayer,
    Finish,
    ToggleStartStop,
}

struct ChessClockView(ChessClock);

impl Application for ChessClockView {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = Rules;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let clock = ChessClock::new(flags);
        (Self(clock), Command::none())
    }

    fn title(&self) -> String {
        String::from("Chess Clock")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SwitchPlayer => {
                self.0.switch_player();
            },
            Message::Finish => {
                self.0.finish();
            },
            Message::Update => {
                self.0.update();
            },
            Message::ToggleStartStop => {
                if self.0.status() == Status::Stopped {
                    self.0.start();
                } else {
                    self.0.stop();
                }
            }
        }
        Command::none()
    }

    // Subscription is used to update the clock every 100 milliseconds
    // and to listen for keyboard input
    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let update = match self.0.status() {
            Status::Running => {
                time::every(Duration::from_millis(100))
                    .map(|_| Message::Update)
            }
            _ => Subscription::none(),
        };

        let keypress = keyboard::on_key_press(
            move |key: keyboard::Key, _modifiers: keyboard::Modifiers| {
                match key {
                    keyboard::Key::Named(keyboard::key::Named::Space) => {
                        Some(Message::SwitchPlayer)
                    },
                    keyboard::Key::Named(keyboard::key::Named::Enter) => {
                        Some(Message::ToggleStartStop)
                    },
                    keyboard::Key::Named(keyboard::key::Named::Backspace) => {
                        Some(Message::Finish)
                    }
                    _ => None
                }
            }
        );


        Subscription::batch(vec![
            update,
            keypress
        ])
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, Renderer> {
        let clock = &self.0;
        let (time1, time2) = clock.read();
        let p1_time_str = format!("{}", DurationDisplay::from(time1));
        let p2_time_str = format!("{}", DurationDisplay::from(time2));
        let active_player = clock.active_player();

        let header_text = button(
            text("Chess Clock")
                .horizontal_alignment(alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center)
                .size(constants::HEADER_SIZE)
                .style(theme::Text::Color(Color::BLACK))
        )
        .width(2 * constants::WIDTH + constants::SPACING)
        .style(theme::Button::Text);

        let time_button = |
            time: &str,
            active_player: bool,
            finished: &Status,
        | {
            button(
                text(time)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center)
                    .size(constants::CLOCK_TEXT_SIZE)
                    .style(theme::Text::Color(Color::BLACK))
            )
            .width(constants::WIDTH)
            .height(constants::HEIGHT)
            .style(
                match (active_player, finished) {
                    (true, Status::Finished) => theme::Button::Positive,
                    (false, Status::Finished) => theme::Button::Destructive,
                    (true, _) => theme::Button::Primary,
                    (false, _) => theme::Button::Secondary,
                }
            )
        };

        let player_text = |name: &str| {
            text(name)
                .width(constants::WIDTH)
                .horizontal_alignment(alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center)
                .size(constants::TEXT_SIZE)
        };

        let p1_text = player_text("Player 1");
        let p2_text = player_text("Player 2");

        let p1_time_button = time_button(
            &p1_time_str,
            active_player == State::Player1,
            &clock.status()
        );
        let p2_time_button = time_button(
            &p2_time_str,
            active_player == State::Player2,
            &clock.status()
        );

        let p1 = column![
            p1_text, p1_time_button
        ]
        .spacing(constants::SPACING);

        let p2 = column![
            p2_text, p2_time_button
        ]
        .spacing(constants::SPACING);

        let content = column![
            header_text,
            row![
                p1, p2
            ].spacing(constants::SPACING)
        ].spacing(constants::SPACING);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
