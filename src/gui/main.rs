use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
#[cfg(debug_assertions)]
use dioxus_hot_reload::Config as HotConfig;

fn main() {
    #[cfg(debug_assertions)]
    hot_reload_init!(HotConfig::new()
        .with_paths(&["src/gui"])
        .with_rebuild_command("cargo run --bin perk_solver_gui --features=\"gui\""));
    let config = Config::new().with_window(
        WindowBuilder::default()
            .with_title(format!("Perk solver {}", env!("CARGO_PKG_VERSION")))
            .with_inner_size(dioxus_desktop::LogicalSize::new(800.0, 500.0))
            .with_resizable(true),
    );

    dioxus_desktop::launch_cfg(app, config);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        style { include_str!("./css/common.css") }
        div { id: "wrapper",
            main {
                span {"hello"}
                span {"hello"}
            }

        }

    ))
}
