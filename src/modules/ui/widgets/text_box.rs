use iced::widget::text_editor;
use text_editor::{Action, Style, Content, Binding};
use iced::border::Radius;
use iced::keyboard::key::{Key, Named};
use iced::{Background, Border, Element, Length, Theme};

use super::super::theme;

use log::{error, warn, info, debug, trace};

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
            Key::Named(Named::Delete) => {
                if key_press.modifiers.control() {
                    // Ctrl+Delete deletes the word to the right
                    let word_length = count_characters(value, false);
                    let delete_sequence = (0..word_length)
                        .map(|_| Binding::Delete)
                        .collect();
                    
                    Some(Binding::Sequence(delete_sequence))                
                } else {
                    // Regular Delete behavior
                    Some(Binding::Delete)
                }
            },
            Key::Named(Named::Backspace) => {
                if key_press.modifiers.control() {
                    // Create a sequence of Backspace operations to delete a word
                    let word_length = count_characters(value, true);
                    let backspace_sequence = (0..word_length)
                        .map(|_| Binding::Backspace)
                        .collect();
                    
                    Some(Binding::Sequence(backspace_sequence))
                } else {
                    // Regular Backspace behavior
                    Binding::from_key_press(key_press)
                }
            },
            Key::Named(Named::Enter) => {
                if key_press.modifiers.shift() {
                    // Let Shift+Enter use the default behavior
                    Binding::from_key_press(key_press)
                } else {
                    // Custom behavior for Enter without Shift
                    Some(Binding::Custom(on_enter.clone()))
                }
            },
            _ => Binding::from_key_press(key_press)
        }
    })
    .into()
}

fn count_characters(content: &Content, is_backspace: bool) -> usize {
    let mut count = 0;

    let (line_number, index) = content.cursor_position();

    let lines_vec = content.lines().collect::<Vec<_>>();
    let reversed = lines_vec[0..=line_number].iter().rev();


    for line in reversed {
        let text: &str = &*line;
        let size: i32 = text.chars().count().try_into().unwrap();
        trace!("Size of {} is {}", text, size);

        if size <= 1 {
            count += 1;
            continue;
        }

        let increment: i32 = if is_backspace {-1} else {1};
        let mut index: i32 = index.try_into().unwrap();

        if is_backspace { index += increment;}

        while index + increment >= -1 && index + increment <= size {
            let char = text.chars().nth(index as usize).unwrap_or(char::from_u32(0).unwrap());
            trace!("Browsing char {} at index {}", char, index);

            if char.is_alphanumeric() || char == '_' {
                count += 1;
                index += increment;
            } else {
                break;
            }
        }
        break;
    }

    debug!("Counter {} chars to delete", count);
    return count;
}
