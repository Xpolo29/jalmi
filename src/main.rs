mod modules;

use iced::widget::text_editor::{Action, Content};
use iced::widget::{combo_box, container, ComboBox, row, column};
use iced::{Length, Theme};
use iced::Element;
use iced::Padding;

use modules::ui::widgets::text_box::rounded_text_box;

struct AppState {
    text_content: Content,
    dropdown_state: combo_box::State<String>,
    selected_model: Option<String>,
}

impl Default for AppState {
    fn default() -> Self {
        let mut options: Vec<String> = Vec::new();
        for option in ["111111111111111111111111111111111", "2", "3333333333333333333333333333333333333333333333333333333", "4", "55555555555555555555555555555555555555555555555555555555555555555555555555555"] {
            options.push(option.to_string());
        }
        
        Self {
            dropdown_state: combo_box::State::new(options),
            selected_model: None,
            text_content: Content::new(),
        }
    }
}

#[derive(Clone, Debug)]
enum Message {
    TextInputChanged(Action),
    OptionSelected(String),
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
        Message::OptionSelected(selected) => {
            state.selected_model = Some(selected);
        }
        _ => {}
    }
    iced::Task::none()
}

fn view(state: &AppState) -> Element<'_, Message> {
    
    // Text input
    let text_box = rounded_text_box(
        &state.text_content,
         "Type something here...",
          Message::TextInputChanged
        );

    // Model selection
    let dropdown: ComboBox<'_, String, Message, Theme> = combo_box(
        &state.dropdown_state,
        "Select an option...",
        state.selected_model.as_ref(),
        Message::OptionSelected
    );
    
    column![
        // Fill top to bottom (TODO: replace it with the interface)
        container(column![]).height(Length::Fill),
        
        // Model selection widget (takes 2/5 of screen width)
        container(
            row![
            container(dropdown).width(Length::FillPortion(2)),
            container(column![]).width(Length::FillPortion(5))
            ])
        .padding(Padding::from(10)),

        // Text input
        container(text_box).padding(Padding::from(10)),
    ]
    .height(Length::Fill)
    .into()
}