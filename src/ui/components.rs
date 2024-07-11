use iced::{
    alignment,
    widget::{button, container, text},
    Element, Length,
};

use crate::Message;

pub fn centered_text<'a>(string: String) -> Element<'a, Message> {
    container(
        text(string)
            .horizontal_alignment(alignment::Horizontal::Center)
            .size(50),
    )
    // .width(Length::Fill)
    // .height(Length::Fill)
    .center_y()
    .into()
}

pub fn song_button<'a>(string: String, message: Message) -> Element<'a, Message> {
    container(button(text(string)).on_press(message))
        // .width(Length::Fill)
        // .height(Length::Shrink)
        .center_y()
        .into()
}
