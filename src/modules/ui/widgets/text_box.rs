use iced::widget::text_editor;
use text_editor::{Action, Style, Content, Binding};
use iced::border::Radius;
use iced::keyboard::key::{Key, Named};
use iced::{Background, Border, Element, Length, Theme};

use super::super::theme;

pub fn rounded_text_box<'a, Message>(
    value: &'a Content,
    placeholder: &'a str,
    on_change: impl Fn(Action) -> Message + 'a,
    on_enter: Message
) -> Element<'a, Message> 
where
    Message: Clone + 'static,
{
    text_editor(value)
    .placeholder(placeholder)
    .on_action(move |action| on_change(action))
    .padding(10)
    .height(Length::Fixed(100.))
    .style(|_, status| {
        let default = text_editor::default(&theme::get_default_theme(), status);
        Style {
            border: Border {
                radius: Radius::from(15.0),
                color: theme::border(),
                ..default.border
            },
            background: Background::Color(theme::text_input()),
            value: theme::font_color(),
            ..default
        }
    })
    .key_binding(move |key_press| {
        match key_press.key {
            Key::Named(Named::Delete) => Some(Binding::Delete),
            Key::Named(Named::Enter) => Some(Binding::Custom(on_enter.clone())),
            _ => Binding::from_key_press(key_press)
        }
    })
    .into()

}
