mod modules;

use iced::widget::text_editor::{Action, Content};
use iced::widget::{Column, container};
use iced::Length;
use iced::Element;
use iced::Padding;

use modules::ui::widgets::text_box::rounded_text_box;

#[derive(Default)]
struct AppState {
    text_content: Content,
}

#[derive(Clone, Debug)]
enum Message {
    TextInputChanged(Action)
}

fn main() -> iced::Result {
    iced::application("Jalmi (Just Another Language Model Interface)", update, view)
        .theme(|_| modules::ui::theme::get_default_theme())
        .run()
}

fn update(state: &mut AppState, message: Message) -> iced::Task<Message> {
    match message {
        Message::TextInputChanged(action) => {
            state.text_content.perform(action);
        },
    }
    iced::Task::none()
}

fn view(state: &AppState) -> Element<'_, Message> {
    
    let spacer = container(Column::new()).height(Length::Fill);

    let text_box = rounded_text_box(
        &state.text_content,
         "Type something here...",
          Message::TextInputChanged
        );
    
    Column::with_children(vec![
        spacer.into(),
        container(text_box).padding(Padding::from(10)).into(),
    ])
    .height(Length::Fill)
    .into()
}