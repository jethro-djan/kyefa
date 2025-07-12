use iced::{Element, Length, Alignment};
use iced::widget::{column, text, button, container, Space};
use crate::app::{DashboardState, Message, DashboardMessage};

pub fn reports_analytics_view(state: &DashboardState) -> Element<'_, Message> {
    let header = text("Reports & Analytics")
        .size(24)
        .style(|theme| iced::widget::text::primary(theme));
    
    let content = column![
        text("View reports and analytics"),
        Space::with_height(20),
        button("Student Performance Report")
            .on_press(Message::Dashboard(DashboardMessage::NavigateToHome)), // Dummy action
        button("Payment Summary")
            .on_press(Message::Dashboard(DashboardMessage::NavigateToHome)), // Dummy action
        button("Teacher Activity Report")
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
