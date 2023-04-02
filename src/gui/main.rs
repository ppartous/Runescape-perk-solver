// On Windows platform, don't show a console when opening the app.
// #![windows_subsystem = "windows"]

mod args;
mod prelude;

use prelude::*;
use derivative::Derivative;
use args::{Args, ArgsMessage};

use iced::{Element, Sandbox, Theme, Settings};
use iced::widget::column;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Derivative, Debug)]
#[derivative(Default)]
struct App {
    args: Args
}

#[derive(Debug, Clone)]
enum AppMessage {
    ArgsMessage(ArgsMessage)
}

impl Sandbox for App {
    type Message = AppMessage;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Perk solver")
    }

    fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::ArgsMessage(message) => self.args.update(message)
        }
    }

    fn view(&self) -> Element<AppMessage> {
        column![
            self.args.view().map(AppMessage::ArgsMessage)
        ]
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}