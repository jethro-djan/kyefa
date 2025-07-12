use iced::{Element, Length, Alignment};
use iced::widget::{column, text, button, container, Space};
use crate::app::{DashboardState, Message, DashboardMessage};

pub fn user_access_view(state: &DashboardState) -> Element<'_, Message> {
    let header = text("User Access Management")
        .size(24)
        .style(|theme| iced::widget::text::primary(theme));
    
    let content = column![
        text("Manage user accounts and permissions"),
        Space::with_height(20),
        button("Create New User")
            .on_press(Message::Dashboard(DashboardMessage::NavigateToHome)),
        button("Manage Permissions")
            .on_press(Message::Dashboard(DashboardMessage::NavigateToHome)),
        button("Deactivate Users")
            .on_press(Message::Dashboard(DashboardMessage::NavigateToHome)),
    ]
    .spacing(10)
    .padding(20)
    .align_x(Alignment::Start);
    
    container(
        column![header, Space::with_height(20), content]
            .padding(20)
            .spacing(10)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
