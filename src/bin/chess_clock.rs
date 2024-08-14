// Use iced to create a GUI for the chess clock
use rusty_chess_clock::{ChessClock, DurationDisplay, State, Status};
use iced::{
    alignment,
    executor,
    keyboard,
    theme::{self, Theme},
    time,
    widget::{button, column, container, row, text},
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
    ChessClockView::run(Settings::default())
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
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let clock = ChessClock::default();
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
        const WIDTH: u16 = 400;
        const HEIGHT: u16 = 400;
        const SPACING: u16 = 20;
        const CLOCK_TEXT_SIZE: u16 = 50;
        const TEXT_SIZE: u16 = 30;
        const HEADER_SIZE: u16 = 50;


        let clock = &self.0;
        let (time1, time2) = clock.read();
        let p1_time_str = format!("{}", DurationDisplay::from(time1));
        let p2_time_str = format!("{}", DurationDisplay::from(time2));
        let active_player = clock.active_player();

        let header_text = button(
            text("Chess Clock")
                .horizontal_alignment(alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center)
                .size(HEADER_SIZE)
                .style(theme::Text::Color(Color::BLACK))
        )
        .width(2 * WIDTH + SPACING)
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
                    .size(CLOCK_TEXT_SIZE)
                    .style(theme::Text::Color(Color::BLACK))
            )
            .width(WIDTH)
            .height(HEIGHT)
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
                .width(WIDTH)
                .horizontal_alignment(alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center)
                .size(TEXT_SIZE)
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
        .spacing(SPACING);

        let p2 = column![
            p2_text, p2_time_button
        ]
        .spacing(SPACING);

        let content = column![
            header_text,
            row![
                p1, p2
            ].spacing(SPACING)
        ].spacing(SPACING);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
