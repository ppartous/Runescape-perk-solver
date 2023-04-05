// On Windows platform, don't show a console when opening the app.
// #![windows_subsystem = "windows"]

mod args;
mod prelude;

use args::{AppArgs, ArgsMessage};
use derivative::Derivative;
use indicatif::HumanCount;
use itertools::Itertools;
use perk_solver::{result, Solver, SolverMetadata};
use prelude::*;

use iced::widget::{
    button, column, horizontal_space, progress_bar, row, scrollable, text, vertical_space, Column,
};
use iced::{
    theme, Alignment, Application, Color, Command, Element, Length, Padding, Settings, Theme,
};

use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

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
    finish_duration: Option<Duration>,
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
                        self.finish_duration.take();
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
                        self.finish_duration = Some(self.start_time.elapsed());
                    }
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<AppMessage> {
        scrollable(
            column![
                self.args.view().map(AppMessage::ArgsMessage),
                if let Some(solver) = self.solver.as_ref() {
                    let progress_percent = 100.0 * self.progress as f32 / solver.total_combination_count as f32;
                    let dur = if let Some(dur) = self.finish_duration {
                        dur.as_secs()
                    } else {
                        self.start_time.elapsed().as_secs()
                    };

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
                        },
                        if let Some(result) = self.solver_result.as_done_ref() {
                            make_result_table(result, &solver.args)
                        } else {
                            column![].into()
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
                },
            ]
        )
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

    fn as_done_ref(&self) -> Option<&Vec<Vec<ResultLine>>> {
        match self {
            SolverResult::Done(x) => Some(&x),
            _ => None,
        }
    }
}

fn text_mono<'a>(x: impl ToString) -> Element<'a, AppMessage> {
    text(x).font(fonts::Hack).into()
}

fn text_mono_color<'a>(x: impl ToString, color: Color) -> Element<'a, AppMessage> {
    text(x).font(fonts::Hack).style(color).into()
}

fn make_result_table<'a>(
    best_per_level: &Vec<Vec<ResultLine>>,
    args: &Args,
) -> Element<'a, AppMessage> {
    let mut list = vec![vertical_space(10).into()];
    let mut tbl = vec![];

    if let Some((best_gizmo_index, best_attempt_index, best_price_index)) =
        result::get_best_of_each(best_per_level)
    {
        let best_wanted_index = match args.sort_type {
            SortType::Gizmo => best_gizmo_index,
            SortType::Attempt => best_attempt_index,
            SortType::Price => best_price_index,
        };

        let best_wanted = &best_per_level[best_wanted_index][0];
        let best_gizmo_prob = best_per_level[best_gizmo_index][0].prob_gizmo;
        let best_attempt_prob = best_per_level[best_attempt_index][0].prob_attempt;
        let best_price = best_per_level[best_price_index][0].price;

        let val = match args.sort_type {
            SortType::Price => result::format_price(best_per_level[best_wanted_index][0].price),
            SortType::Gizmo => format!(
                "{}%",
                result::format_float(best_per_level[best_wanted_index][0].prob_gizmo)
            ),
            SortType::Attempt => format!(
                "{}%",
                result::format_float(best_per_level[best_wanted_index][0].prob_attempt)
            ),
        };
        list.push(text_mono(format!(
            "Best combination at level {}:\n {:<8}: {}",
            best_wanted.level,
            val,
            MaterialName::vec_to_string(best_wanted.mat_combination.as_ref())
        )));

        if args.result_depth > 1 {
            list.push(vertical_space(10).into());
            list.push(text_mono("Alts:"));
            for alt in result::find_best_alts(best_per_level, args) {
                let val = match args.sort_type {
                    SortType::Price => result::format_price(alt.price),
                    SortType::Gizmo => format!("{}%", result::format_float(alt.prob_gizmo)),
                    SortType::Attempt => format!("{}%", result::format_float(alt.prob_attempt)),
                };
                list.push(text_mono(format!(
                    " {:<8} @lvl {}: {}",
                    val,
                    alt.level,
                    MaterialName::vec_to_string(alt.mat_combination.as_ref())
                )));
            }
            list.push(vertical_space(10).into());
        }

        tbl.push(text_mono(
            "      ┌───────┬───────────────────────────┬───────────┐",
        ));
        tbl.push(text_mono(
            "      │       │      Probability (%)      │           │",
        ));
        tbl.push(text_mono(
            "      │ Level ├─────────────┬─────────────┤   Price   │",
        ));
        tbl.push(text_mono(
            "      │       │    Gizmo    │   Attempt   │           │",
        ));
        tbl.push(text_mono(
            "      ├───────┼─────────────┼─────────────┼───────────┤",
        ));

        for (i, line) in best_per_level.iter().enumerate() {
            let (r1, g1, b1) = result::get_color(line[0].prob_gizmo / best_gizmo_prob);
            let (r2, g2, b2) = result::get_color(line[0].prob_attempt / best_attempt_prob);
            let (r3, g3, b3) = result::get_color(best_price / line[0].price);

            tbl.push(
                row![
                    text_mono("      │ "),
                    text_mono(format!("{:>4}", line[0].level)),
                    text_mono("  │  "),
                    text_mono_color(
                        format!("{:>9}", result::format_float(line[0].prob_gizmo)),
                        Color::from_rgb8(r1, g1, b1)
                    ),
                    text_mono("  │  "),
                    text_mono_color(
                        format!("{:>9}", result::format_float(line[0].prob_attempt)),
                        Color::from_rgb8(r2, g2, b2)
                    ),
                    text_mono("  │ "),
                    text_mono_color(
                        format!("{:>9}", result::format_price(line[0].price)),
                        Color::from_rgb8(r3, g3, b3)
                    ),
                    text_mono(format!(
                        " │{}",
                        if i == best_wanted_index { " <====" } else { "" }
                    )),
                ]
                .spacing(-0.6)
                .into(),
            );
        }

        tbl.push(text_mono(
            "      └───────┴─────────────┴─────────────┴───────────┘",
        ));
    } else {
        list.push(text_mono(
            "No material combination found that can produce this gizmo.",
        ));
    }

    column![Column::with_children(list), Column::with_children(tbl)]
        .align_items(Alignment::Center)
        .into()
}
