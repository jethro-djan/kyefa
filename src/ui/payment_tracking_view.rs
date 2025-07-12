use iced::{Element, Length, Alignment};
use iced::widget::{column, text, button, container, Space};
use crate::app::{DashboardState, Message, DashboardMessage};

pub fn payment_tracking_view(state: &DashboardState) -> Element<'_, Message> {
    let header = text("Payment Tracking")
        .size(24)
        .style(|theme| iced::widget::text::primary(theme));
    
    let content = column![
        text("Track and manage payments"),
        Space::with_height(20),
        button("Record Payment")
            .on_press(Message::Dashboard(DashboardMessage::NavigateToHome)), // Dummy action
        button("View Payment History")
            .on_press(Message::Dashboard(DashboardMessage::NavigateToHome)), // Dummy action
        button("Generate Invoice")
            .on_press(Message::Dashboard(DashboardMessage::NavigateToHome)), // Dummy action
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
