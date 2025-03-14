use iced::widget::text_editor;
use text_editor::{Action, Style, Content, Binding};
use iced::border::Radius;
use iced::keyboard::key::{Key, Named};

use iced::{Border, Length, Element, Theme};

pub fn rounded_text_box<'a, Message>(
    value: &'a Content,
    placeholder: &'a str,
    on_change: impl Fn(Action) -> Message + 'a,
) -> Element<'a, Message> 
where
    Message: Clone + 'static,
{
    text_editor(value)
    .placeholder(placeholder)
    .on_action(move |action| on_change(action))
    .padding(10)
    .height(Length::Fixed(100.))
    .style(|theme: &Theme, status| {
        let default = text_editor::default(theme, status);
        
        Style {
            border: Border {
                radius: Radius::from(15.0),
                ..default.border
            },
            ..default
        }
    })
    .key_binding(|key_press| {
        match key_press.key {
            Key::Named(Named::Delete) => Some(Binding::Delete),
            _ => Binding::from_key_press(key_press)
        }
    })
    .into()

}
