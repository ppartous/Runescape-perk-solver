// On Windows platform, don't show a console when opening the app.
// #![windows_subsystem = "windows"]

mod numeric_input;
use std::str::FromStr;
use derivative::Derivative;

use numeric_input::numeric_input;
use iced::{
    Element,
    Application,
    Theme,
    Settings,
    // Length,
    alignment::*,
    widget::{
        column,
        row,
        // progress_bar,
        // text_input,
        text,
        // button,
        // checkbox,
        // container,
        horizontal_space,
        pick_list,
    }, Command
};
use perk_solver::prelude::PerkName;
use strum::VariantNames;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Derivative, Debug)]
#[derivative(Default)]
struct App {
    perk: Option<PerkName>,
    rank: Option<u32>,
    perk_two: Option<PerkName>,
    rank_two: Option<u32>,
    #[derivative(Default(value = "[Some(1), Some(137)]"))]
    invention_level: [Option<u32>; 2],
    fuzzy: bool
}

#[derive(Debug, Clone)]
enum Message {
    PerkChanged(&'static str),
    RankChanged(Option<u32>),
    PerkTwoChanged(&'static str),
    RankTwoChanged(Option<u32>),
    InventionLevelChanged([Option<u32>; 2])
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self::default(),
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("A simple Progressbar")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PerkChanged(x) => self.perk = Some(PerkName::from_str(x).unwrap_or_default()),
            Message::RankChanged(x) => self.rank = x,
            Message::PerkTwoChanged(x) => {
                self.perk_two = Some(PerkName::from_str(x).unwrap_or_default());
                self.fuzzy = x == "Any";
            },
            Message::RankTwoChanged(x) => self.rank_two = x,
            Message::InventionLevelChanged(x) =>
                self.invention_level = [x[0].or(self.invention_level[0]), x[1].or(self.invention_level[1])]
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        column![
            row![
                text("Perk one: "),
                pick_list(&PerkName::VARIANTS[1..], self.perk.map(|x| x.into()), Message::PerkChanged)
                    .placeholder("Pick one"),
                horizontal_space(10),
                numeric_input(self.rank, |x| Message::RankChanged(x.map(|y| y.min(6).max(1))))
                    .placeholder("Rank")
            ]
            .align_items(Alignment::Center),
            row![
                text("Perk two: "),
                pick_list([&["Any"], PerkName::VARIANTS].concat(), self.perk_two.map(|x| if self.fuzzy {"Any"} else {x.into()}), Message::PerkTwoChanged)
                    .placeholder("Pick one"),
                horizontal_space(10),
                numeric_input(self.rank_two, |x| Message::RankTwoChanged(x.map(|y| y.min(6).max(1))))
                    .placeholder("Rank")
            ]
            .align_items(Alignment::Center),
            row![
                text("Invention level: "),
                numeric_input(self.invention_level[0], |x| Message::InventionLevelChanged([x.map(|y| y.min(137).max(1)), None])),
                text(" — "),
                numeric_input(self.invention_level[1], |x| Message::InventionLevelChanged([None, x.map(|y| y.min(137).max(1))]))
            ]
            .align_items(Alignment::Center),
            // progress_bar(0.0..=100.0, self.value),
        ]
        .spacing(10)
        .padding(20)
        .into()
    }
}