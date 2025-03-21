mod modules;

use iced::{
    border::Radius, overlay::menu, task::{self, Handle}, time::{self, Duration}, widget::{
        column, combo_box, container, row, scrollable, text_editor::{Action, Content}, text_input::{self, Status}, Column, ComboBox, Text
    }, window::{self, Mode}, Alignment, Background, Border, Element, Length, Padding, Settings, Size, Subscription, Task, Theme
};


use modules::{
    ui::{
        widgets::{
            bubble::text_bubble,
            text_box::rounded_text_box,
            toggle_button::toggle_button,
            status::status_display,
        },
        theme,
    },
    config::config,
    llm::llm::{LocalAiClient, ModelStatus, is_model_active, get_status},
};
use log::{error, warn, info, debug, trace};
use env_logger::{Builder, Env};
use std::env;
use std::thread::sleep;

struct AppState {
    text_content: Content,
    dropdown_state: combo_box::State<String>,
    selected_model: Option<String>,
    history: Vec<(String, bool)>,
    client: LocalAiClient,
    is_streaming: Option<Handle>,
    model_status: Option<ModelStatus>,
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
            is_streaming: None,
            model_status: None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    None,
    TextInputChanged(Action),
    OptionSelected(String),
    Generate,
    Request,
    ReceivedChunk(String),
    StreamCompleted,
    StreamStop,
    Regenerate,
    UnloadModel, 
    LoadModel,
    CheckLoaded,
}

fn main() -> iced::Result {
    env::set_var("RUST_LOG", "jalmi=trace,none");
    colog::init();

    info!("App is starting");
    iced::application("Jalmi (Just Another Language Model Interface)", update, view)
        .theme(|_| theme::get_default_theme())
        .window(window::Settings {
            min_size: Some(Size::new(300.0, 400.0)), 
            ..Default::default()
        })
        .run()
}

// Update Model method
fn update(state: &mut AppState, message: Message) -> iced::Task<Message> {
    // debug!("Received message {:?}", message);
    match message {
        // Text input
        Message::TextInputChanged(action) => {
            state.text_content.perform(action);
        },
        // Model selection
        Message::OptionSelected(selected) => {
            state.selected_model = Some(selected);
            return Task::done(Message::CheckLoaded);
        },
        Message::Request => {
            // Add empty AI response to history
            state.history.push((String::new(), true));
            
            // Call stream_completion and return the task
            let model = &state.selected_model.as_ref().unwrap();
            let (task, handle) = state.client.stream_completion(&state.history, model);

            state.is_streaming = Some(handle);
            return task;
        },
        // Enter pressed for example
        Message::Generate => {
            let text = state.text_content.text();
            
            if !text.trim().is_empty() && state.selected_model.is_some() && !state.is_streaming.is_some() {
                // Add user prompt
                state.history.push((text.to_string(), false));
                state.text_content = Content::new();
                
                return Task::done(Message::Request)            
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
        Message::StreamCompleted => {
            state.is_streaming = None;
        },
        Message::StreamStop => {
            warn!("Stream aborted !");
            if let Some(handle) = state.is_streaming.as_mut() {
                handle.abort();
                state.is_streaming = None;
            } 
        }
        Message::Regenerate => {
            state.history.pop();

            return Task::done(Message::Request)            
        },
        Message::UnloadModel => {
            if is_model_active(state.model_status.clone()) {
                
            }
        },
        // Load if not streaming and model not loaded and selected model is not null
        Message::LoadModel => {
            let model = &state.selected_model.as_ref().unwrap();
            let (task, handle) = state.client.stream_completion(&state.history, model);

            let timeout_task = Task::perform(
                async move {
                    sleep(Duration::from_millis(100));
                    handle.abort();
                },
                |_| Message::None
            );
        
            return Task::batch(vec![
                task,
                timeout_task
            ])
        },
        Message::CheckLoaded => {
            debug!("Cheking if model is loaded !");
            let client = state.client.clone();
            let model = state.selected_model.clone();

            
            let status = client.clone().check_status(model.clone());
            if status != state.model_status{
                state.model_status = status;
            }
        },
        _ => {}
    }
    iced::Task::none()
}

// Update UI method
fn view(state: &AppState) -> Element<'_, Message> {
    trace!("VIEW");
    // text input
    let text_box = rounded_text_box(
        &state.text_content,
        "Type something here...",
        Message::TextInputChanged,
        Message::Generate,
        state.is_streaming.is_none(),
    );

    // Send query to llm button
    let send_button =  toggle_button(
        state.is_streaming.is_none(),
        ("Send", Message::Generate),
        ("Stop", Message::StreamStop),
        theme::bottom(),
        theme::error(),
    );

    // Retry button
    let retry_button =  toggle_button(
        state.is_streaming.is_none() && state.history.len() > 0,
        ("Retry", Message::Regenerate),
        ("...", Message::None),
        theme::bubble_left(),
        theme::background(),
    );

    // Unload models button
    // Load if not streaming and model not loaded and selected model is not null
    let unload_button =  toggle_button(
        state.is_streaming.is_none() 
            && !is_model_active(state.model_status.clone())
            && state.selected_model.is_some(),
        ("Load", Message::LoadModel),
        (if state.selected_model.is_some() {"Unload"} else {"Select a model first"}, Message::UnloadModel),
        theme::model(),
        theme::background(),
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

    // Model status display
    let status: container::Container<'_, Message> = status_display(state.model_status.clone());

    // Conversation bubbles
    let mut bubbles = Vec::new();
    for (msg, left) in (&state.history).into_iter() {
        let bubble = text_bubble(msg, *left);
        bubbles.push(bubble.into());
    }
    let conversation = Column::with_children(bubbles);

    // Main view architecture
    column![
        // Top
        container(
            scrollable(
                conversation
            ).spacing(10)
        )
        .height(Length::Fill)
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
            container(
                row![
                    container(dropdown).padding(Padding::from(0).right(10)).height(Length::Fill),
                    status.padding(Padding::from(0).right(10)).height(Length::Fill),
                    container(unload_button).align_x(Alignment::Center).height(Length::Fill).padding(Padding::from(0).left(10).right(10)),
                ]
            )
            .padding(10)
            .height(50),

            row![
                // Text input
                container(text_box).padding(Padding::from(10).right(5)),

                // Send button and retry button
                container(
                    column![
                        container(send_button).padding(5).height(Length::FillPortion(1)).width(Length::Fill),
                        container(retry_button).padding(5).height(Length::FillPortion(1)).width(Length::Fill),
                    ]
                )
                .padding(Padding::from(10).left(5))
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .height(100)
                .width(100)
            ]

        ])
        .align_bottom(Length::Fill)
        .height(Length::Shrink)
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