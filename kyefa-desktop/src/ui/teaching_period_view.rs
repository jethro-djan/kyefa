use iced::{Element, Length, Alignment};
use iced::widget::{column, text, button, container, Space};
use crate::app::{DashboardState, Message, DashboardMessage};

pub fn teaching_period_view(state: &DashboardState) -> Element<'_, Message> {
    let header = text("Teaching Period Management")
        .size(24)
        .style(|theme| iced::widget::text::primary(theme));
    
    let content = column![
        text("Manage teaching periods and schedules"),
        Space::with_height(20),
        button("Create New Period")
            .on_press(Message::Dashboard(DashboardMessage::NavigateToHome)), // Dummy action
        button("View Schedule")
            .on_press(Message::Dashboard(DashboardMessage::NavigateToHome)), // Dummy action
        button("Mark Attendance")
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
