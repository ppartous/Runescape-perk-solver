// On Windows platform, don't show a console when opening the app.
// #![windows_subsystem = "windows"]

mod args;
mod prelude;

use args::{AppArgs, ArgsMessage};
use derivative::Derivative;
use indicatif::HumanCount;
use perk_solver::{Solver, SolverMetadata};
use prelude::*;
use itertools::Itertools;

use iced::widget::{button, column, progress_bar, row, text, horizontal_space};
use iced::{theme, Alignment, Application, Command, Element, Length, Settings, Theme, Padding};

use std::sync::atomic::Ordering;
use std::time::Instant;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Derivative)]
#[derivative(Default)]
struct App {
    progress: u64,
    args: AppArgs,
    solver: Option<SolverMetadata>,
    solver_result: SolverResult,
    #[derivative(Default(value = "Instant::now()"))]
    start_time: Instant,
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
                self.progress = 0;
                match setup(&self.args) {
                    Ok(solver) => {
                        self.error_text.take();
                        self.solver = Some(solver.meta.clone());
                        self.solver_result = SolverResult::Future(std::thread::spawn(move || {
                            solver.run().join().unwrap()
                        }));
                        self.start_time = Instant::now();
                        return Command::perform(
                            App::progress_bar_update_timer(),
                            AppMessage::ProgressUpdateMessage,
                        );
                    }
                    Err(err) => {
                        self.error_text = Some(err);
                        self.solver.take();
                        self.solver_result.take();
                    }
                };
            }
            AppMessage::CancelMessage => {
                if let Some(solver) = self.solver.as_ref() {
                    solver.cancel_signal.store(true, Ordering::SeqCst);
                }
                if let SolverResult::Future(fut) = self.solver_result.take() {
                    fut.join().ok();
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
                    } else {
                        if let SolverResult::Future(fut) = self.solver_result.take_fut() {
                            self.solver_result = SolverResult::Done(fut.join().unwrap());
                        }
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
                let progress_percent = 100.0 * self.progress as f32 / solver.total_combination_count as f32;
                let dur = self.start_time.elapsed().as_secs();

                column![
                    if progress_percent == 100.0  {
                        button("Start!").on_press(AppMessage::StartMessage)
                    } else {
                        button("Cancel")
                            .on_press(AppMessage::CancelMessage)
                            .style(theme::Button::Destructive)
                    },
                    row![
                        text(
                            format!(
                                "[{:02}:{:02}:{:02}]",
                                dur / 3600,
                                (dur % 3600) / 60,
                                (dur % 60)
                            )
                        ),
                        progress_bar(0.0..=100.0, progress_percent),
                        text(
                            format!(
                                "{} / {} ({} %)",
                                HumanCount(self.progress),
                                HumanCount(solver.total_combination_count),
                                progress_percent as u64
                            )
                        )
                    ]
                    .padding(20)
                    .spacing(10)
                    .align_items(Alignment::Center),
                    row![
                        text("Possible materials:").font(fonts::bold::Roboto),
                        horizontal_space(5.0),
                        text(
                            solver.materials.conflict
                                .iter()
                                .chain(&solver.materials.no_conflict)
                                .map(|x| x.to_string())
                                .join(", ")
                        )
                    ]
                    .padding(Padding::from([0, 20])),
                    if !solver.args.exclude.is_empty() {
                        row![
                            text("Excluded materials:").font(fonts::bold::Roboto),
                            horizontal_space(5.0),
                            text(
                                solver.args.exclude
                                    .iter()
                                    .map(|x| x.to_string())
                                    .join(", ")
                            )
                        ]
                        .padding(Padding::from([0, 20]))
                    } else {
                        row![]
                    }
                ]
                .align_items(Alignment::Center)
                .width(Length::Fill)
            } else {
                column![
                    button("Start!").on_press(AppMessage::StartMessage),
                    if let Some(err) = self.error_text.as_ref() {
                        text(err)
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

#[derive(Default)]
enum SolverResult {
    #[default]
    None,
    Future(std::thread::JoinHandle<Vec<Vec<ResultLine>>>),
    Done(Vec<Vec<ResultLine>>),
}

impl SolverResult {
    fn take(&mut self) -> Self {
        std::mem::take(self)
    }

    fn take_fut(&mut self) -> Self {
        match self {
            SolverResult::Future(_) => self.take(),
            _ => SolverResult::None,
        }
    }
}
