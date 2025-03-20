use iced::widget::shader::wgpu::naga::back;
use iced::widget::text_editor;
use text_editor::{Action, Style, Content, Binding, Motion};
use iced::border::Radius;
use iced::keyboard::key::{Key, Named};
use iced::{Background, Border, Element, Length, Color};

use std::cmp::max;

use super::super::theme;

use log::{error, warn, info, debug, trace};

pub fn rounded_text_box<'a, Message>(
    value: &'a Content,
    placeholder: &'a str,
    on_change: impl Fn(Action) -> Message + 'a,
    on_enter: Message,
    is_active: bool,
) -> Element<'a, Message> 
where
    Message: Clone + 'static,
{
    text_editor(value)
    .placeholder(placeholder)
    .on_action(move |action| on_change(action))
    .padding(10)
    .height(Length::Fixed(100.))
    .style(move |_, status| {
        let default: Style = text_editor::default(&theme::get_default_theme(), status);
        Style {
            border: Border {
                radius: Radius::from(15.0),
                color: theme::border(),
                ..default.border
            },
            background: Background::Color(if is_active {
                theme::text_input()
            } else {
                let color = theme::text_input();
                Color::from_rgba(color.r * 0.7, color.g * 0.7, color.b * 0.7, color.a)
            }),
                        value: theme::font_color(),
            ..default
        }
    })
    .key_binding(move |key_press| {

        match key_press.key {
            Key::Named(Named::Delete) => {
                if key_press.modifiers.control() {
                    let count = count_characters(value, true);
                    let mut backspace_sequence: Vec<Binding<Message>> = (0..count)
                        .map(|_| Binding::Select(Motion::Right))
                        .collect();

                    backspace_sequence.push(Binding::Select(Motion::WordRight));
                    backspace_sequence.push(Binding::Delete);

                    Some(Binding::Sequence(backspace_sequence))              
                } else {
                    Some(Binding::Delete)
                }
            },
            Key::Named(Named::Backspace) => {
                if key_press.modifiers.control() {
                    let count = count_characters(value, true);
                    let mut backspace_sequence: Vec<Binding<Message>> = (0..count)
                        .map(|_| Binding::Select(Motion::Left))
                        .collect();

                    backspace_sequence.push(Binding::Select(Motion::WordLeft));
                    backspace_sequence.push(Binding::Delete);

                    Some(Binding::Sequence(backspace_sequence))
                } else {
                    Binding::from_key_press(key_press)
                }
            },
            Key::Named(Named::Enter) => {
                if key_press.modifiers.shift() {
                    Binding::from_key_press(key_press)
                } else {
                    Some(Binding::Custom(on_enter.clone()))
                }
            },
            _ => Binding::from_key_press(key_press)
        }
    })
    .into()
}



fn count_characters(content: &Content, is_backspace: bool) -> usize {
    let mut count: usize = 0;

    let (line_number, mut index) = content.cursor_position();
    let lines_vec = content.lines().collect::<Vec<_>>();
    let reversed = lines_vec[0..=line_number].iter().rev();

    let mut first_line = true;
    for line in reversed {

        let text: &str = &*line;
        let size: i32 = text.chars().count().try_into().unwrap();
        let mut local_count: i32 = 0;

        if !first_line {index = max(size, 0) as usize;}

        //trace!("Size of {} is {}", text, size);

        let increment: i32 = if is_backspace {-1} else {1};
        let mut index: i32 = index.try_into().unwrap();
        if is_backspace { index += increment;}

        // Delete until on a word or end of line
        while index + increment >= -1 && index + increment <= size {
            let char = text.chars().nth(index as usize).unwrap_or(char::from_u32(0).unwrap());
            //trace!("Browsing char {} at index {}", char, index);
            if char == ' ' {
                count += 1;
                index += increment;
                local_count += 1;
            } else {
                break;
            }
        }

        // Delete until on a line
        if size - local_count <= 1 {
            //trace!("Size of {} is {} with count {}", text, size, local_count);
            count += 1;
            first_line = false;
            continue;
        }
        break;
    }

    //debug!("Counted {} chars to delete", count);
    return count;
}
