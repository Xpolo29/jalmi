use iced::widget::combo_box;

pub fn custom_dropdown<'a, T, Message>(
    state: &'a combo_box::State<T>,
    placeholder: &str,
    selected: Option<&T>,
    on_selected: impl Fn(T) -> Message + 'static,
) -> iced::Element<'a, Message>
where
    T: Clone + std::fmt::Display + 'static,
    Message: Clone + 'static,
{
    combo_box(
        state,
        placeholder,
        selected,
        on_selected,
    )
    .into()
}
