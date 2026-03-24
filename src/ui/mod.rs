pub mod styles;
pub mod toolbar;

use iced::widget::{column, container, Space};
use iced::{Element, Length};

use crate::core::message::Message;
use crate::core::state::State;

pub fn view(_state: &State) -> Element<'_, Message> {
    let toolbar = toolbar::render();

    // CONTENT PRINCIPAL
    let content = container(Space::new())
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill);

    // OUTPUT: TOOLBAR + CONTENT
    column![toolbar, Space::new().height(10), content].into()
}