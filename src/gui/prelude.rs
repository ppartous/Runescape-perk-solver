pub use perk_solver::prelude::*;

#[allow(non_upper_case_globals)]
pub mod fonts {
    use iced::Font;

    pub static Hack: Font = Font::External {
        name: "Hack",
        bytes: include_bytes!("../../fonts/Hack/Hack-Regular.ttf"),
    };

    pub mod bold {
        use super::*;

        pub static Roboto: Font = Font::External {
            name: "Roboto bold",
            bytes: include_bytes!("../../fonts/roboto/Roboto-Bold.ttf"),
        };
    }
}
