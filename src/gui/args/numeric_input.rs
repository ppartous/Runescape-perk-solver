use iced::alignment::Alignment;
use iced::widget::{self, button, row,  text_input, column, vertical_space, svg};
use iced::{Element, Length};
use iced_lazy::{self, Component};

pub struct NumericInput<Message> {
    value: Option<u32>,
    placeholder: Option<&'static str>,
    on_change: Box<dyn Fn(Option<u32>) -> Message>,
}

pub fn numeric_input<Message>(
    value: Option<u32>,
    on_change: impl Fn(Option<u32>) -> Message + 'static,
) -> NumericInput<Message> {
    NumericInput::new(value, on_change)
}

#[derive(Debug, Clone)]
pub enum Event {
    InputChanged(String),
    IncrementPressed,
    DecrementPressed,
}

impl<Message> NumericInput<Message> {
    pub fn new(
        value: Option<u32>,
        on_change: impl Fn(Option<u32>) -> Message + 'static,
    ) -> Self {
        Self {
            value,
            placeholder: None,
            on_change: Box::new(on_change),
        }
    }

    pub fn placeholder(mut self, value: &'static str) -> Self {
        self.placeholder = Some(value);
        self
    }
}

impl<Message, Renderer> Component<Message, Renderer> for NumericInput<Message>
where
    Renderer: iced_native::text::Renderer + 'static,
    Renderer: iced_native::svg::Renderer,
    Renderer::Theme: widget::button::StyleSheet
        + widget::text_input::StyleSheet
        + widget::text::StyleSheet
        + iced_native::widget::svg::StyleSheet
{
    type State = ();
    type Event = Event;

    fn update(
        &mut self,
        _state: &mut Self::State,
        event: Event,
    ) -> Option<Message> {
        match event {
            Event::IncrementPressed => Some((self.on_change)(Some(
                self.value.unwrap_or_default().saturating_add(1),
            ))),
            Event::DecrementPressed => Some((self.on_change)(Some(
                self.value.unwrap_or_default().saturating_sub(1),
            ))),
            Event::InputChanged(value) => {
                if value.is_empty() {
                    Some((self.on_change)(None))
                } else {
                    value
                        .parse()
                        .ok()
                        .map(Some)
                        .map(self.on_change.as_ref())
                }
            }
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Event, Renderer> {
        let button = |icon, on_press| {
            button(
                svg(icon)
                    .width(Length::Fill)
                    .height(Length::Fill)
            )
            .width(20)
            .height(15)
            .padding(2)
            .on_press(on_press)
        };

        row![
            text_input(
                self.placeholder.unwrap_or(""),
                self.value
                .as_ref()
                .map(u32::to_string)
                .as_deref()
                .unwrap_or(""),
                Event::InputChanged,
            )
            .padding(5)
            .width(50),
            column![
                button(iced_native::svg::Handle::from_memory(include_bytes!("../../../images/up arrow.svg").as_ref()), Event::IncrementPressed),
                vertical_space(1),
                button(iced_native::svg::Handle::from_memory(include_bytes!("../../../images/down arrow.svg").as_ref()), Event::DecrementPressed),
            ],
        ]
        .align_items(Alignment::Center)
        .spacing(1)
        .into()
    }
}

impl<'a, Message, Renderer> From<NumericInput<Message>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'static + iced_native::text::Renderer,
    Renderer: iced_native::svg::Renderer,
    Renderer::Theme: widget::button::StyleSheet
        + widget::text_input::StyleSheet
        + widget::text::StyleSheet
        + iced_native::widget::svg::StyleSheet
{
    fn from(numeric_input: NumericInput<Message>) -> Self {
        iced_lazy::component(numeric_input)
    }
}