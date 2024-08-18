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
    Settings,
    Subscription
};
use std::time::Duration;

fn main() -> iced::Result {
    let _rules = Rules::default();
    Pages::run(Settings::default())
}

mod constants {
    pub const WIDTH: u16 = 400;
    pub const HEIGHT: u16 = 400;
    pub const SPACING: u16 = 20;
    pub const CLOCK_TEXT_SIZE: u16 = 50;
    pub const TEXT_SIZE: u16 = 30;
    pub const HEADER_SIZE: u16 = 50;

    pub mod settings {
        pub const TEXTBOX_WIDTH: u16 = 250;
        pub const INPUT_ELEMENT_SPACING: u16 = 10;
        pub const BETWEEN_ELEMENT_SPACING: u16 = INPUT_ELEMENT_SPACING * 2;
    }
}

#[derive(Debug)]
struct Pages {
    settings: ChessClockSettings,
    clock: Option<ChessClockView>,
}

#[derive(Debug)]
enum PagesMessage {
    SettingsMessage(SettingsMessage),
    ClockMessage(ChessClockViewMessage),
}

impl Application for Pages {
    type Message = PagesMessage;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                settings: ChessClockSettings::new(),
                clock: None,
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        match self.clock {
            Some(_) => "Chess Clock".to_string(),
            None => "Chess Clock - Select Settings".to_string()
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Self::Message::ClockMessage(ChessClockViewMessage::ResetClock) => {
                self.clock = None;
                self.settings = ChessClockSettings::new();
            },
            Self::Message::SettingsMessage(
                SettingsMessage::InitialiseClock
            ) => {
                self.clock = Some(
                    ChessClockView(ChessClock::new(self.settings.rules.clone()))
                );
            }
            Self::Message::SettingsMessage(message) => {
                self.settings.update(message);
            }
            Self::Message::ClockMessage(message) => {
                self.clock.as_mut().unwrap().update(message);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        match &self.clock {
            Some(clock) => clock.view().map(Self::Message::ClockMessage),
            None => self.settings.view().map(Self::Message::SettingsMessage)
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        match &self.clock {
            Some(clock) => {
                clock.subscription().map(Self::Message::ClockMessage)
            },
            None => {
                self.settings.subscription().map(Self::Message::SettingsMessage)
            }
        }
    }

}

#[derive(Debug)]
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
    InitialiseClock,
}

impl ChessClockSettings {
    /// Produce the time selector elements for a player
    ///
    /// # Arguments
    ///
    /// * `player` - The player to select the time for
    ///
    /// # Returns
    ///
    /// * A tuple of three elements:
    ///     * A label for the time input element
    ///     * A text input for the time input element
    ///     * A printout of the time
    fn time_selector(&self, player: State) -> (
        Element<SettingsMessage>,
        Element<SettingsMessage>,
        Element<SettingsMessage>
    ) {
        let index = player.index();
        let time_str = &self.time_strings[index];
        let placeholder = format!("Enter {} Time (minutes)", player);

        let time_input = text_input(&placeholder, time_str)
            .on_input(move |input| SettingsMessage::ChangeTime(player, input))
            .width(constants::settings::TEXTBOX_WIDTH);


        let time_label = text(format!("{} Time", player).as_str());

        let time_printout = text(
            DurationDisplay::from(self.rules.get_time(player)).to_string()
        );

        (time_label.into(), time_input.into(), time_printout.into())
    }

    /// Produce the increment selector element
    ///
    /// # Returns
    ///
    /// * A tuple of three elements:
    ///     * A label for the text input element
    ///     * A text input element
    ///     * A printout of current value of the increment
    fn increment_selector(&self) -> (
        Element<SettingsMessage>,
        Element<SettingsMessage>,
        Element<SettingsMessage>
    ) {
        let increment = &self.increment_string;
        let placeholder = "Enter Increment (seconds)";

        let increment_input = text_input(placeholder, increment)
            .on_input(SettingsMessage::ChangeIncrement)
            .width(constants::settings::TEXTBOX_WIDTH);

        let increment_label = text("Increment");

        let increment = self.rules.get_increment();
        let printout_text = DurationDisplay::from(increment).to_string();
        let increment_printout = text(printout_text);

        (
            increment_label.into(),
            increment_input.into(),
            increment_printout.into()
        )
    }

    /// Produce the timing method selector element
    ///
    /// # Returns
    ///
    /// * An array of three elements:
    ///     * A label for the timing method selector element
    ///     * A pick list for the timing method selector element
    ///     * A printout of the current timing method
    fn timing_method_selector(&self) -> (
        Element<SettingsMessage>,
        Element<SettingsMessage>,
        Element<SettingsMessage>
    ) {
        let pick_list = pick_list(
            &TimingMethod::ALL[..],
            Some(self.rules.get_timing_method()),
            SettingsMessage::ChangeTimingMethod
        );

        let label = text("Timing Method");
        let printout = text(self.rules.get_timing_method().to_string());

        (label.into(), pick_list.into(), printout.into())
    }

    /// Produce the active player selector element
    ///
    /// # Returns
    ///
    /// * A tuple of three elements:
    ///     * A label for the active player selector element
    ///     * A pick list for the active player selector element
    ///     * A printout of the current active player
    fn active_player_selector(&self) -> (
        Element<SettingsMessage>,
        Element<SettingsMessage>,
        Element<SettingsMessage>
    ) {
        let pick_list = pick_list(
            &State::ALL[..],
            Some(self.rules.get_starter()),
            SettingsMessage::ChangeActivePlayer
        );

        let label = text("Active Player");
        let printout = text(self.rules.get_starter().to_string());
        (label.into(), pick_list.into(), printout.into())
    }

    /// Produce the start button element
    ///
    /// # Returns
    ///
    /// * A button that produces a message to start the clock and move to the
    ///   chess clock view page
    fn start_button(&self) -> Element<SettingsMessage> {
        let button = button(text("Start clock"))
            .on_press(SettingsMessage::InitialiseClock)
            .style(theme::Button::Primary)
            .padding(constants::SPACING);

        button.into()
    }

    /// Create a new instance of the chess clock settings
    fn new() -> Self {
        let rules = Rules::default();
        Self {
            rules,
            time_strings: ["".to_string(), "".to_string()],
            increment_string: "".to_string(),
        }
    }

    fn update(&mut self, message: SettingsMessage) {
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
            },

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
            },

            SettingsMessage::ChangeTimingMethod(timing_method) => {
                self.rules.set_timing_method(timing_method);
            },

            SettingsMessage::ChangeActivePlayer(starter) => {
                self.rules.set_starter(starter);
            },

            _ => {}
        }
    }

    fn subscription(&self) -> Subscription<SettingsMessage> {
        // Start the clock when enter is pressed
        let keypress = keyboard::on_key_press(
            move |key: keyboard::Key, _modifiers: keyboard::Modifiers| {
                match key.as_ref() {
                    keyboard::Key::Named(keyboard::key::Named::Enter) => {
                        Some(SettingsMessage::InitialiseClock)
                    },
                    _ => None
                }
            }
        );

        keypress
    }

    fn view(&self) -> Element<SettingsMessage> {
        let header_text = text("Chess Clock")
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center)
            .size(constants::HEADER_SIZE)
            .style(theme::Text::Color(Color::BLACK))
            .width(Length::Fill);

        let p1_time_elements = self.time_selector(State::Player1);
        let p2_time_elements = self.time_selector(State::Player2);
        let increment_elements = self.increment_selector();
        let timing_method_elements = self.timing_method_selector();
        let active_player_elements = self.active_player_selector();
        let start_button = self.start_button();

        let p1_time_row = row![
            p1_time_elements.0, p1_time_elements.1
        ].format(
            alignment::Alignment::Center,
            constants::settings::INPUT_ELEMENT_SPACING
        );
        let p2_time_row = row![
            p2_time_elements.0, p2_time_elements.1
        ].format(
            alignment::Alignment::Center,
            constants::settings::INPUT_ELEMENT_SPACING
        );

        let timing_row = row![
            p1_time_row, p2_time_row
        ].format(
            alignment::Alignment::Center,
            constants::settings::BETWEEN_ELEMENT_SPACING
        );

        let increment_row = row![
            increment_elements.0, increment_elements.1
        ].format(
            alignment::Alignment::Center,
            constants::settings::INPUT_ELEMENT_SPACING
        );
        let timing_method_row = row![
            timing_method_elements.0,
            timing_method_elements.1,
        ].format(
            alignment::Alignment::Center,
            constants::settings::INPUT_ELEMENT_SPACING
        );
        let active_player_row = row![
            active_player_elements.0,
            active_player_elements.1,
        ].format(
            alignment::Alignment::Center,
            constants::settings::INPUT_ELEMENT_SPACING
        );

        let settings_row = row![
            increment_row, timing_method_row, active_player_row
        ].format(
            alignment::Alignment::Center,
            constants::settings::BETWEEN_ELEMENT_SPACING
        );

        let summary_box = row![
            column![
                text("Player 1 Time"), text("Player 2 Time"),
                text("Increment"), text("Timing Method"), text("Starter")
            ].format(
                alignment::Alignment::Start,
                constants::settings::INPUT_ELEMENT_SPACING
            ).width(110),
            column![
                p1_time_elements.2, p2_time_elements.2,
                increment_elements.2, timing_method_elements.2,
                active_player_elements.2
            ].format(
                alignment::Alignment::End,
                constants::settings::INPUT_ELEMENT_SPACING
            ).width(65)
        ].format(
            alignment::Alignment::Center,
            constants::settings::BETWEEN_ELEMENT_SPACING
        );


        container(column![
            header_text,
            timing_row,
            settings_row,
            summary_box,
            start_button
        ].align_items(alignment::Alignment::Center)
            .spacing(constants::SPACING))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone)]
enum ChessClockViewMessage {
    Update,
    SwitchPlayer,
    Finish,
    ToggleStartStop,
    ResetClock,
}

#[derive(Debug)]
struct ChessClockView(ChessClock);

impl ChessClockView {
    fn update(&mut self, message: ChessClockViewMessage) {
        match message {
            ChessClockViewMessage::SwitchPlayer => {
                self.0.switch_player();
            },
            ChessClockViewMessage::Finish => {
                self.0.finish();
            },
            ChessClockViewMessage::Update => {
                self.0.update();
            },
            ChessClockViewMessage::ToggleStartStop => {
                if self.0.status() == Status::Stopped {
                    self.0.start();
                } else {
                    self.0.stop();
                }
            },
            _ => {}
        }
    }

    // Subscription is used to update the clock every 100 milliseconds
    // and to listen for keyboard input
    fn subscription(&self) -> iced::Subscription<ChessClockViewMessage> {
        let update = match self.0.status() {
            Status::Running => {
                time::every(Duration::from_millis(100))
                    .map(|_| ChessClockViewMessage::Update)
            }
            _ => Subscription::none(),
        };

        let keypress = keyboard::on_key_press(
            move |key: keyboard::Key, _modifiers: keyboard::Modifiers| {
                match key.as_ref() {
                    keyboard::Key::Named(keyboard::key::Named::Space) => {
                        Some(ChessClockViewMessage::SwitchPlayer)
                    },
                    keyboard::Key::Named(keyboard::key::Named::Enter) => {
                        Some(ChessClockViewMessage::ToggleStartStop)
                    },
                    keyboard::Key::Named(keyboard::key::Named::Backspace) => {
                        Some(ChessClockViewMessage::Finish)
                    }
                    keyboard::Key::Character("q") => {
                        Some(ChessClockViewMessage::ResetClock)
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

    fn view(&self) -> Element<'_, ChessClockViewMessage, Theme, Renderer> {
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

// Define a trait for formatting elements
trait Format {
    fn format(self, alignment: alignment::Alignment, spacing: u16) -> Self;
}

impl<'a> Format for iced::widget::Row<'a, SettingsMessage> {
    /// Format the row to have a center alignment and a certain spacing
    /// between the elements
    fn format(self, alignment: alignment::Alignment, spacing: u16) -> Self {
        self.align_items(alignment).spacing(spacing)
    }
}

impl<'a> Format for iced::widget::Column<'a, SettingsMessage> {
    fn format(self, alignment: alignment::Alignment, spacing: u16) -> Self {
        self.align_items(alignment).spacing(spacing)
    }
}
