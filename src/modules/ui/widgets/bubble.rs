use iced::widget::{Container, Text, container, row, column};
use iced::{Theme, Background, Border};
use iced::border::Radius;
use iced::{Length, Padding};
use iced::alignment::Horizontal::{Left, Right};

use super::super::theme;

/// Creates a text bubble from the given string
pub fn text_bubble<'a, Message>(content: &'a str, align_left: bool) -> Container<'a, Message, Theme> 
where
    Message: 'static,
{
    // Create the text widget with the content
    let text = Text::new(content);

    // Wrap the text in a container with bubble styling
    let bubble = container(text)
        .padding(10)
        
        .style(move |_| {
            let default = container::Style::default();
            let background_color = if align_left { theme::bubble_left() } else { theme::bubble_right() };
            
            container::Style {
                background: Some(Background::Color(background_color)),
                border: Border {
                    radius: Radius::from(15.0),
                    width: 1.0,
                    color: theme::border(),
                },
                text_color: Some(theme::font_color()),
                ..default
            }
        });

    // Wrap the bubble into a aligned container
    let alignment = if align_left { Left } else { Right };

    let bubble = container(bubble)
        .height(Length::Shrink)
        .width(Length::FillPortion(4))
        .align_x(alignment);

    // Add an empty widget to the left or right so it doesn't take whole screen space
    if align_left {
        container(
            row![
                bubble,
                container(column![]).width(Length::FillPortion(1))
            ])
        .padding(Padding::from(10))
        .width(Length::Fill)
    } else {
        container(
            row![
                container(column![]).width(Length::FillPortion(1)),
                bubble
            ])
        .padding(Padding::from(10))
        .width(Length::Fill)
    }
}

