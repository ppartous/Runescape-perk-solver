// On Windows platform, don't show a console when opening the app.
// #![windows_subsystem = "windows"]

mod args;
mod prelude;
mod utils;

use args::{AppArgs, ArgsMessage};
use derivative::Derivative;
use perk_solver::{Solver, SolverMetadata};
use prelude::*;

use iced::widget::{button, column, progress_bar, row, text};
use iced::{theme, Alignment, Application, Command, Element, Length, Settings, Theme};

use std::sync::atomic::Ordering;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Derivative)]
#[derivative(Default)]
struct App {
    progress: u64,
    args: AppArgs,
    solver: Option<SolverMetadata>,
    solver_result: Option<std::thread::JoinHandle<Vec<Vec<ResultLine>>>>,
    error_text: Option<String>,
}

#[derive(Debug, Clone)]
enum AppMessage {
    ArgsMessage(ArgsMessage),
    StartMessage,
    CancelMessage,
    ProgressUpdateMessage(()),
}

#[rustfmt::skip::macros(column, row)]
impl Application for App {
    type Message = AppMessage;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<AppMessage>) {
        colored::control::set_override(false);
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        format!("Perk solver {}", env!("CARGO_PKG_VERSION"))
    }

    fn update(&mut self, message: AppMessage) -> Command<AppMessage> {
        match message {
            AppMessage::ArgsMessage(message) => self.args.update(message),
            AppMessage::StartMessage => {
                self.solver.take();
                self.progress = 0;
                let solver = match setup(&self.args) {
                    Ok(solver) => solver,
                    Err(err) => {
                        self.error_text = Some(err);
                        return Command::none();
                    }
                };
                self.error_text.take();
                self.solver = Some(solver.meta.clone());
                self.solver_result = Some(std::thread::spawn(move || solver.run().join().unwrap()));
                return Command::perform(
                    App::progress_bar_update_timer(),
                    AppMessage::ProgressUpdateMessage,
                );
            }
            AppMessage::CancelMessage => {
                if let Some(solver) = self.solver.as_ref() {
                    solver.cancel_signal.store(true, Ordering::SeqCst);
                }
                if let Some(res) = self.solver_result.take() {
                    res.join().ok();
                }
                self.solver.take();
                self.progress = 0;
            }
            AppMessage::ProgressUpdateMessage(_) => {
                if let Some(solver) = self.solver.as_ref() {
                    if self.progress < solver.total_combination_count {
                        self.progress = solver.bar_progress.load(Ordering::Relaxed);
                        return Command::perform(
                            App::progress_bar_update_timer(),
                            AppMessage::ProgressUpdateMessage,
                        );
                    }
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<AppMessage> {
        column![
            self.args.view().map(AppMessage::ArgsMessage),
            if let Some(solver) = self.solver.as_ref() {
                let progress = 100.0 * self.progress as f32 / solver.total_combination_count as f32;
                column![
                    if progress == 100.0  {
                        button("Start!").on_press(AppMessage::StartMessage)
                    } else {
                        button("Cancel")
                            .on_press(AppMessage::CancelMessage)
                            .style(theme::Button::Destructive)
                    },
                    row![
                        progress_bar(0.0..=100.0, progress),
                        text(format!("({} %)", progress as usize)).width(70.0)
                    ]
                    .padding(20)
                    .spacing(10)
                    .align_items(Alignment::Center),
                    text(format!(
                        "{} / {}",
                        utils::format_int(self.progress as i64),
                        utils::format_int(solver.total_combination_count as i64)
                    ))
                ]
                .align_items(Alignment::Center)
                .width(Length::Fill)
            } else {
                column![
                    button("Start!").on_press(AppMessage::StartMessage),
                    if let Some(err) = self.error_text.as_ref() {
                        text(format!("Error: {}", err))
                    } else {
                        text("").height(0.0)
                    }
                ]
                .align_items(Alignment::Center)
                .width(Length::Fill)
                .spacing(5)
            }
        ]
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

impl App {
    async fn progress_bar_update_timer() {
        let len = std::time::Duration::from_millis(100);
        tokio::time::sleep(len).await;
    }
}

fn setup(appargs: &AppArgs) -> Result<Solver, String> {
    let args = Args::create(&appargs.to_cli())?;
    let data = Data::load();
    let solver = perk_solver::Solver::new(args, data)?;
    Ok(solver)
}
