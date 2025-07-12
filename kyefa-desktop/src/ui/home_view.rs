use iced::{Element, Length};
use iced::widget::{column, text, Space};

use crate::app::{Message, DashboardState};

pub fn home_view(state: &DashboardState) -> Element<'_, Message> {
    column![
        text(format!("Welcome, {} {}", 
            state.active_user.name.first_name, 
            state.active_user.name.surname))
            .size(40),
        Space::with_height(Length::Fixed(20.0)),
        text("Here's a summary of your recent activities and key metrics:").size(20),
        Space::with_height(Length::Fixed(20.0)),
        text("- Total Students: 150").size(18),
        text("- Active Teachers: 12").size(18),
        text("- Upcoming Payouts: GHS 5,000.00").size(18),
        Space::with_height(Length::Fixed(30.0)),
        text("Recent Changes:").size(20),
        Space::with_height(Length::Fixed(10.0)),
        text("- Added new student: John Doe").size(18),
        text("- Updated Mr. Kobi's period share").size(18),
    ]
    .padding(20)
    .spacing(10)
    .into()
}
