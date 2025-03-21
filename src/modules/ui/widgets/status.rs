use iced::{
    widget::{container, text, Container, Text}, Alignment, Background, Border, Color, Element, Length, Padding, Theme
};
use iced::border::Radius;
use crate::{ModelStatus, get_status, theme};

pub fn status_display<'a, Message>(model_status: &'a Option<ModelStatus>) -> Container<'a, Message, Theme> {
    // Get the status text
    let status_text = get_status(model_status);
    
    // Create the text widget
    let text = Text::new(format!("   Status: {}  ", status_text));
    
    // Create a container with rounded corners
    let status_box = container(text)
        .padding(10)
        .width(Length::Shrink)
        .style(move |_| {
            // Determine background color based on status
            let background_color = match model_status {
                Some(ModelStatus::Ready) => Color::from_rgb(0.2, 0.8, 0.2),     // Green
                Some(ModelStatus::Starting) => Color::from_rgb(0.9, 0.6, 0.1),  // Orange
                Some(ModelStatus::Stopping) => Color::from_rgb(0.9, 0.3, 0.1),  // Red-Orange
                Some(ModelStatus::Stopped) => Color::from_rgb(0.9, 0.3, 0.1),  // Red-Orange
                None => Color::from_rgb(0.7, 0.7, 0.7),                         // Light Gray
            };
            
            container::Style {
                background: Some(Background::Color(background_color)),
                border: Border {
                    radius: Radius::from(15.0),  // Rounded corners
                    width: 1.0,
                    color: theme::border(),  // Dark border
                },
                text_color: Some(theme::font_color()),  // White text for contrast
                ..container::Style::default()
            }
        })
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);
    
    status_box
}
