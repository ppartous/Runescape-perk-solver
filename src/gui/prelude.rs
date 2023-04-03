use iced::Font;
pub use perk_solver::prelude::*;

pub static ROBOTO_BOLD: Font = Font::External {
    name: "Roboto bold",
    bytes: include_bytes!("../../fonts/roboto/Roboto-Bold.ttf"),
};
