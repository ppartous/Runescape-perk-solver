pub use perk_solver::prelude::*;

#[allow(non_upper_case_globals)]
pub mod fonts {
    pub mod bold {
        pub static Roboto: iced::Font = iced::Font::External {
            name: "Roboto bold",
            bytes: include_bytes!("../../fonts/roboto/Roboto-Bold.ttf"),
        };
    }
}