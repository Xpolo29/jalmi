// gui.rs
use crate::api::{ApiClient, CompletionRequest};
use crate::config::Config;
use iced::{
    button, executor, scrollable, text_input, Application, Button, Column, Command, Container,
    Element, Length, Row, Scrollable, Settings, Text, TextInput,
};

#[derive(Debug, Clone)]
pub enum Message {
    ModelSelected(String),
    PromptChanged(String),
    SendPressed,
    ResponseReceived(Result<String, String>),
}

pub struct LlmFrontend {
    config: Config,
    api_client: ApiClient,
    models: Vec<String>,
    selected_model: Option<String>,
    prompt: String,
    response: String,
    prompt_input: text_input::State,
    send_button: button::State,
    response_scroll: scrollable::State,
    model_buttons: Vec<button::State>,
}

impl Application for LlmFrontend {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Config;

    fn new(config: Config) -> (Self, Command<Message>) {
        let models = config.models.keys().cloned().collect::<Vec<_>>();
        let model_buttons = vec![button::State::new(); models.len()];

        (
            Self {
                config,
                api_client: ApiClient::new(),
                models,
                selected_model: None,
                prompt: String::new(),
                response: String::new(),
                prompt_input: text_input::State::new(),
                send_button: button::State::new(),
                response_scroll: scrollable::State::new(),
                model_buttons,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("LLM Frontend")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ModelSelected(model) => {
                self.selected_model = Some(model);
                Command::none()
            }
            Message::PromptChanged(value) => {
                self.prompt = value;
                Command::none()
            }
            Message::SendPressed => {
                if let Some(model) = &self.selected_model {
                    let request = CompletionRequest {
                        model: model.clone(),
                        prompt: self.prompt.clone(),
                        max_tokens: 1024,
                        temperature: 0.7,
                    };

                    let api_client = self.api_client.clone();
                    return Command::perform(
                        async move {
                            match api_client.get_completion(request).await {
                                Ok(response) => {
                                    if let Some(choice) = response.choices.first() {
                                        Ok(choice.text.clone())
                                    } else {
                                        Err("No response text received".to_string())
                                    }
                                }
                                Err(e) => Err(format!("Error: {}", e)),
                            }
                        },
                        Message::ResponseReceived,
                    );
                }
                Command::none()
            }
            Message::ResponseReceived(result) => {
                match result {
                    Ok(text) => self.response = text,
                    Err(e) => self.response = format!("Error: {}", e),
                }
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        // Model selection
        let models_row = Row::with_children(
            self.models
                .iter()
                .enumerate()
                .map(|(i, model)| {
                    Button::new(
                        &mut self.model_buttons[i],
                        Text::new(model).size(14),
                    )
                    .on_press(Message::ModelSelected(model.clone()))
                    .into()
                })
                .collect(),
        )
        .spacing(10);

        // Prompt input
        let input = TextInput::new(
            &mut self.prompt_input,
            "Enter your prompt here...",
            &self.prompt,
            Message::PromptChanged,
        )
        .padding(10);

        // Send button
        let send_button = Button::new(&mut self.send_button, Text::new("Send"))
            .on_press(Message::SendPressed)
            .width(Length::Units(100));

        // Response area
        let response_area = Scrollable::new(&mut self.response_scroll)
            .push(Text::new(&self.response))
            .width(Length::Fill)
            .height(Length::Fill);

        // Layout
        let content = Column::new()
            .push(Text::new("Select a model:").size(16))
            .push(models_row)
            .push(input)
            .push(send_button)
            .push(Text::new("Response:").size(16))
            .push(response_area)
            .spacing(20)
            .padding(20);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

