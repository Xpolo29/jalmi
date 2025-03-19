mod modules;

use iced::{
    overlay::menu, widget::{
        column, combo_box, container, row, scrollable, text_editor::{Action, Content}, text_input::{self, Status}, Column, ComboBox
    }, 
    Background, Element, Length, Padding, Theme, Border,
    border::Radius,
};

use modules::{
    ui::{
        widgets::{
            bubble::text_bubble,
            text_box::rounded_text_box,
        },
        theme,
    },
    config::config,
    llm::llm::LocalAiClient,
};
use log::{error, warn, info, debug, trace};

struct AppState {
    text_content: Content,
    dropdown_state: combo_box::State<String>,
    selected_model: Option<String>,
    history: Vec<(String, bool)>,
    client: LocalAiClient,
}

impl Default for AppState {
    fn default() -> Self {
        let options = config::get_model_list();        
        Self {
            dropdown_state: combo_box::State::new(options),
            selected_model: None,
            text_content: Content::new(),
            history: Vec::new(),
            client: LocalAiClient::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    TextInputChanged(Action),
    OptionSelected(String),
    EnterPressed,
    ReceivedChunk(String),
    StreamCompleted,
}

fn main() -> iced::Result {
    colog::init();
    info!("App is starting");
    iced::application("Jalmi (Just Another Language Model Interface)", update, view)
        .theme(|_| theme::get_default_theme())
        .run()
}

fn update(state: &mut AppState, message: Message) -> iced::Task<Message> {
    trace!("Received message {:?}", message);
    match message {
        // Text input
        Message::TextInputChanged(action) => {
            state.text_content.perform(action);
        },
        // Model selection
        Message::OptionSelected(selected) => {
            state.selected_model = Some(selected);
        },
        // Enter pressed
        Message::EnterPressed => {
            let text = state.text_content.text();
            
            if !text.trim().is_empty() && state.selected_model.is_some() {
                // Add user prompt
                state.history.push((text.to_string(), false));
                state.text_content = Content::new();
                
                // Add empty AI response to history
                state.history.push((String::new(), true));
                
                // Call stream_completion and return the task
                let model = &state.selected_model.as_ref().unwrap();

                return state.client.stream_completion(&state.history, model);
            }
        },
        // Handle streamed chunk
        Message::ReceivedChunk(chunk) => {
            // Append the chunk to the last AI response in history
            if let Some((response, is_ai)) = state.history.last_mut() {
                if *is_ai {
                    response.push_str(&chunk);
                }
            }
        },
        _ => {}
    }
    iced::Task::none()
}


fn view(state: &AppState) -> Element<'_, Message> {
    // Text input
    let text_box = rounded_text_box(
        &state.text_content,
         "Type something here...",
          Message::TextInputChanged,
          Message::EnterPressed,
        );

    // Model selection
    let dropdown: ComboBox<'_, String, Message, Theme> = combo_box(
        &state.dropdown_state,
        "Select an option...",
        state.selected_model.as_ref(),
        Message::OptionSelected
    )
    .input_style(|_, _status| {
        text_input::Style {
            background: Background::Color(theme::model()),
            border: Border {
                radius: Radius::from(15.0),
                width: 1.0,
                color: theme::border(),
            },
            value: theme::font_color(),
            ..text_input::default(&theme::get_default_theme(), Status::Active)
        }
    })
    .menu_style(|_| {
        menu::Style {
            background: Background::Color(theme::model()),
            text_color: theme::font_color(),
            ..menu::default(&theme::get_default_theme())
        }
    });

    // Conversation bubbles
    let mut bubbles = Vec::new();
    
    for (msg, left) in (&state.history).into_iter() {
        let bubble = text_bubble(msg, *left);
        bubbles.push(bubble.into());
    }
    
    let conversation = Column::with_children(bubbles);

    column![
        // Top
        container(
            scrollable(
                conversation
            ).spacing(10)
        )
        .height(Length::FillPortion(4))
        .width(Length::Fill)
        .style(|_| {
            container::Style {
                background: Some(Background::Color(theme::top())),
                ..container::Style::default()
            }
        }),

        // Separator
        container(column![]).height(8),

        // Bottom
        container(column![
            // Model selection
            container(dropdown).width(Length::FillPortion(2)).padding(Padding::from(10)),

            // Text input
            container(text_box).padding(Padding::from(10))

        ])
        .align_bottom(Length::Fill)
        .height(Length::FillPortion(1))
        .style(|_| {
            container::Style {
                background: Some(Background::Color(theme::bottom())),
                ..container::Style::default()
            }
        })
    ]
    .height(Length::Fill)
    .into()
}