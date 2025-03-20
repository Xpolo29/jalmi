use crate::Message;
use iced::widget::{button, text};
use iced::border::Radius;
use iced::{Alignment, Background, Border, Color, Element, Length};
use crate::theme;



pub fn toggle_button<'a>(
    is_active: bool,
    active: (&str, Message),
    inactive: (&str, Message),
    base_color: Color,
    disabled_color: Color,
) -> Element<'a, Message> {
    // Button text changes based on state
    let button_text = if is_active {
        active.0.to_string()
    } else {
        inactive.0.to_string()
    };

    let text = text(button_text)
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .width(Length::Fill);
    
    // Create button with dynamic text and styling
    button(text)
        .on_press(
            if is_active {
                active.1
            } else {
                inactive.1
            }
        )
        .style(move |_, status| {
            let default = button::Style::default();
            
            button::Style {
                border: Border {
                    radius: Radius::from(15.0),
                    color: theme::border(),
                    ..default.border
                },
                background: Some(Background::Color(
                    if is_active {
                        if matches!(status, button::Status::Hovered) {
                            darken_color(base_color, 100.)
                        } else {
                            darken_color(base_color, 60.)
                        }
                    } else {
                        if matches!(status, button::Status::Hovered) {
                            darken_color(disabled_color, 100.)
                        } else {
                            darken_color(disabled_color, 60.)
                        }
                    }
                )),
                text_color: Color::WHITE,
                ..default
            }
        })
        
        .into()
}

fn darken_color(color: Color, factor: f32) -> Color {
    let factor = 1.0 - (factor / 255.0);    Color {
        r: color.r * factor,
        g: color.g * factor,
        b: color.b * factor,
        a: color.a,
    }
}
