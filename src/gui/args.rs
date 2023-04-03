mod numeric_input;

use crate::{PerkName, GizmoType, SortType, Cli};
use numeric_input::numeric_input;

use derivative::Derivative;
use strum::VariantNames;
use iced::{Element, Length, alignment::*};
use iced::widget::{
    column,
    row,
    text_input,
    text,
    checkbox,
    radio,
    container,
    horizontal_space,
    vertical_space,
    pick_list,
    vertical_rule
};

#[derive(Derivative, Debug)]
#[derivative(Default)]
pub struct AppArgs {
    perk: Option<&'static str>,
    rank: Option<u32>,
    perk_two: Option<&'static str>,
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
pub enum ArgsMessage {
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

impl AppArgs {
    pub fn update(&mut self, message: ArgsMessage) {
        match message {
            ArgsMessage::PerkChanged(x) => self.perk = Some(x),
            ArgsMessage::RankChanged(x) => self.rank = x,
            ArgsMessage::PerkTwoChanged(x) => {
                self.perk_two = Some(x);
                self.fuzzy = x == "Any";
            },
            ArgsMessage::RankTwoChanged(x) => self.rank_two = x,
            ArgsMessage::InventionLevelChanged(x) =>
                self.invention_level = [x[0].or(self.invention_level[0]), x[1].or(self.invention_level[1])],
            ArgsMessage::AltsChanged(x) => self.alts = x,
            ArgsMessage::AncientChanged(x) => self.ancient = x,
            ArgsMessage::ExcludeChanged(x) => self.exclude = x,
            ArgsMessage::GizmoTypeChanged(x) => self.gizmo_type = x,
            ArgsMessage::SortTypeChanged(x) => self.sort_type = x
        }
    }

    pub fn view(&self) -> Element<ArgsMessage> {
        let separator = || container(vertical_rule(1)).height(Length::Fixed(160.0));

        column![
            row![
                column![
                    row![
                        text("First perk:").width(Length::Fixed(100.0)),
                        pick_list(&PerkName::VARIANTS[1..], self.perk, ArgsMessage::PerkChanged)
                            .placeholder("Pick one"),
                        horizontal_space(10),
                        numeric_input(self.rank, |x| ArgsMessage::RankChanged(x.map(|y| y.min(6).max(1))))
                            .placeholder("Rank")
                    ]
                    .align_items(Alignment::Center),
                    row![
                        text("Second perk:").width(Length::Fixed(100.0)),
                        pick_list([&["Any"], PerkName::VARIANTS].concat(), self.perk_two, ArgsMessage::PerkTwoChanged)
                            .placeholder("Pick one"),
                        horizontal_space(10),
                        numeric_input(self.rank_two, |x| ArgsMessage::RankTwoChanged(x.map(|y| y.min(6).max(1))))
                            .placeholder("Rank")
                    ]
                    .align_items(Alignment::Center),
                    row![
                        text("Ancient gizmo"),
                        checkbox("", self.ancient, ArgsMessage::AncientChanged),
                    ]
                    .height(Length::Fixed(32.0))
                    .spacing(10)
                    .align_items(Alignment::Center),

                ]
                .spacing(5)
                .padding(20),

                separator(),

                column![
                    text("Gizmo type").font(crate::ROBOTO_BOLD),
                    radio("Weapon", GizmoType::Weapon, Some(self.gizmo_type), ArgsMessage::GizmoTypeChanged),
                    radio("Armour", GizmoType::Armour, Some(self.gizmo_type), ArgsMessage::GizmoTypeChanged),
                    radio("Tool", GizmoType::Tool, Some(self.gizmo_type), ArgsMessage::GizmoTypeChanged),
                ]
                .spacing(5)
                .padding(20),

                separator(),

                column![
                    text("Sort style").font(crate::ROBOTO_BOLD),
                    radio("Price", SortType::Price, Some(self.sort_type), ArgsMessage::SortTypeChanged),
                    radio("Gizmo", SortType::Gizmo, Some(self.sort_type), ArgsMessage::SortTypeChanged),
                    radio("Attempt", SortType::Attempt, Some(self.sort_type), ArgsMessage::SortTypeChanged),
                ]
                .spacing(5)
                .padding(20),

                separator(),

                column![
                    row![
                        text("Invention level:").width(Length::Fixed(120.0)),
                        numeric_input(self.invention_level[0], |x| ArgsMessage::InventionLevelChanged([x.map(|y| y.min(137).max(1)), None])),
                        text(" — "),
                        numeric_input(self.invention_level[1], |x| ArgsMessage::InventionLevelChanged([None, x.map(|y| y.min(137).max(1))]))
                    ]
                    .align_items(Alignment::Center),
                    row![
                        text("Alt count:").width(Length::Fixed(120.0)),
                        numeric_input(self.alts, |x| ArgsMessage::AltsChanged(x.map(|y| y.min(254))))
                    ]
                    .align_items(Alignment::Center),
                    vertical_space(2),
                    text("Exclude filter:"),
                    text_input("E.g.: direct,noxious", &self.exclude, ArgsMessage::ExcludeChanged)
                ]
                .spacing(5)
                .padding(20),
            ],
        ]
        .into()
    }

    pub fn to_cli(&self) -> Cli {
        Cli {
            gizmo_type: self.gizmo_type,
            ancient: self.ancient,
            invention_level: self.invention_level.map(|x| x.unwrap_or(0) as u8).to_vec(),
            command: perk_solver::Commands::Gizmo {
                perk: self.perk.unwrap_or("Empty").to_string(),
                rank: self.rank.unwrap_or(1) as u8,
                perk_two: self.perk_two.map(String::from),
                rank_two: self.rank_two.unwrap_or(1) as u8,
                fuzzy: self.fuzzy,
                exclude: self.exclude.replace(", ", ",").split(",").map(|x| x.to_string()).collect(),
                sort_type: self.sort_type,
                out_file: perk_solver::Args::default().out_file,
                price_file: perk_solver::Args::default().price_file,
                alt_count: self.alts.unwrap_or(0) as u8
            }
        }
    }
}