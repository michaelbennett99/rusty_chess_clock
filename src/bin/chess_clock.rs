// Use iced to create a GUI for the chess clock
use rusty_chess_clock::{ChessClock, DurationDisplay, Status};
use iced::{
    alignment,
    executor,
    keyboard,
    theme::Theme,
    time,
    widget::{button, container, row, text},
    Application,
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
    Start,
    Stop,
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
            Message::Start => {
                self.0.start();
            },
            Message::SwitchPlayer => {
                self.0.switch_player();
            },
            Message::Stop => {
                self.0.stop();
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

        let time_button = | time: &str | {
            button(
                text(time)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center)
                    .size(30)
            )
            .width(200)
            .height(200)
        };

        let p1_time_button = time_button(&p1_time_str);
        let p2_time_button = time_button(&p2_time_str);

        let content = row![
            p1_time_button, p2_time_button
        ]
        .spacing(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
