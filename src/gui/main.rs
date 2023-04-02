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
    // Color,
    Length,
    Font,
    alignment::*,
    widget::{
        column,
        row,
        // progress_bar,
        text_input,
        text,
        // button,
        checkbox,
        radio,
        container,
        horizontal_space,
        vertical_space,
        pick_list,
        // horizontal_rule,
        vertical_rule
    }, Command
};
use perk_solver::prelude::*;
use strum::VariantNames;

static ROBOTO_BOLD: Font = Font::External {
    name: "Roboto bold",
    bytes: include_bytes!("../../fonts/roboto/Roboto-Bold.ttf")
};

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
    fuzzy: bool,
    #[derivative(Default(value = "true"))]
    ancient: bool,
    exclude: String,
    #[derivative(Default(value = "Some(5)"))]
    alts: Option<u32>,
    #[derivative(Default(value = "GizmoType::Weapon"))]
    gizmo_type: GizmoType,
    #[derivative(Default(value = "SortType::Price"))]
    sort_type: SortType
}

#[derive(Debug, Clone)]
enum Message {
    PerkChanged(&'static str),
    RankChanged(Option<u32>),
    PerkTwoChanged(&'static str),
    RankTwoChanged(Option<u32>),
    InventionLevelChanged([Option<u32>; 2]),
    AncientChanged(bool),
    ExcludeChanged(String),
    AltsChanged(Option<u32>),
    GizmoTypeChanged(GizmoType),
    SortTypeChanged(SortType)
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
                self.invention_level = [x[0].or(self.invention_level[0]), x[1].or(self.invention_level[1])],
            Message::AltsChanged(x) => self.alts = x,
            Message::AncientChanged(x) => self.ancient = x,
            Message::ExcludeChanged(x) => self.exclude = x,
            Message::GizmoTypeChanged(x) => self.gizmo_type = x,
            Message::SortTypeChanged(x) => self.sort_type = x
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let separator = || container(vertical_rule(1)).height(Length::Fixed(160.0));

        column![
            row![
                column![
                    row![
                        text("First perk:").width(Length::Fixed(100.0)),
                        pick_list(&PerkName::VARIANTS[1..], self.perk.map(|x| x.into()), Message::PerkChanged)
                            .placeholder("Pick one"),
                        horizontal_space(10),
                        numeric_input(self.rank, |x| Message::RankChanged(x.map(|y| y.min(6).max(1))))
                            .placeholder("Rank")
                    ]
                    .align_items(Alignment::Center),
                    row![
                        text("Second perk:").width(Length::Fixed(100.0)),
                        pick_list([&["Any"], PerkName::VARIANTS].concat(), self.perk_two.map(|x| if self.fuzzy {"Any"} else {x.into()}), Message::PerkTwoChanged)
                            .placeholder("Pick one"),
                        horizontal_space(10),
                        numeric_input(self.rank_two, |x| Message::RankTwoChanged(x.map(|y| y.min(6).max(1))))
                            .placeholder("Rank")
                    ]
                    .align_items(Alignment::Center),
                    row![
                        text("Ancient gizmo"),
                        checkbox("", self.ancient, Message::AncientChanged),
                    ]
                    .height(Length::Fixed(32.0))
                    .spacing(10)
                    .align_items(Alignment::Center),

                ]
                .spacing(5)
                .padding(20),

                separator(),

                column![
                    text("Gizmo type").font(ROBOTO_BOLD),
                    radio("Weapon", GizmoType::Weapon, Some(self.gizmo_type), Message::GizmoTypeChanged),
                    radio("Armour", GizmoType::Armour, Some(self.gizmo_type), Message::GizmoTypeChanged),
                    radio("Tool", GizmoType::Tool, Some(self.gizmo_type), Message::GizmoTypeChanged),
                ]
                .spacing(5)
                .padding(20),

                separator(),

                column![
                    text("Sort style").font(ROBOTO_BOLD),
                    radio("Price", SortType::Price, Some(self.sort_type), Message::SortTypeChanged),
                    radio("Gizmo", SortType::Gizmo, Some(self.sort_type), Message::SortTypeChanged),
                    radio("Attempt", SortType::Attempt, Some(self.sort_type), Message::SortTypeChanged),
                ]
                .spacing(5)
                .padding(20),

                separator(),

                column![
                    row![
                        text("Invention level:").width(Length::Fixed(120.0)),
                        numeric_input(self.invention_level[0], |x| Message::InventionLevelChanged([x.map(|y| y.min(137).max(1)), None])),
                        text(" — "),
                        numeric_input(self.invention_level[1], |x| Message::InventionLevelChanged([None, x.map(|y| y.min(137).max(1))]))
                    ]
                    .align_items(Alignment::Center),
                    row![
                        text("Alt count:").width(Length::Fixed(120.0)),
                        numeric_input(self.alts, |x| Message::AltsChanged(x.map(|y| y.min(254))))
                    ]
                    .align_items(Alignment::Center),
                    vertical_space(2),
                    text("Exclude filter:"),
                    text_input("E.g.: direct,noxious", &self.exclude, Message::ExcludeChanged)
                ]
                .spacing(5)
                .padding(20),
            ],

            // horizontal_rule(10).style(theme::Rule::Custom(Box::new(RuleCustomStyle)))
        ]
        .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

// struct RuleCustomStyle;

// impl iced::widget::rule::StyleSheet for RuleCustomStyle {
//     type Style = Theme;

//     fn appearance(&self, _style: &Self::Style) -> iced::widget::rule::Appearance {
//         iced::widget::rule::Appearance {
//             fill_mode: iced::widget::rule::FillMode::Full,
//             color: iced::Color::from_rgb8(250, 85, 134).into(),
//             radius: 10.0,
//             width: 50
//         }
//     }
// }