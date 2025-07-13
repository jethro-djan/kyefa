use iced::{Element, Length, Color, Alignment, Padding};
use iced::widget::{
    column, row, text, button, container, 
    text_input, scrollable, pick_list, Space
};
use iced::alignment::{Horizontal, Vertical};
use crate::app::{TeachingPeriodManagerState, Message, DashboardMessage, TeachingPeriodMessage};

pub fn teaching_period_view(state: &TeachingPeriodManagerState) -> Element<'_, Message> {
    let header = row![
        text("Teaching Period Management").size(24),
        Space::with_width(Length::Fill),
        button("Import Periods")
            .style(button::secondary)
            .on_press(Message::Dashboard(DashboardMessage::TeachingPeriod(TeachingPeriodMessage::ShowImportDialog))),
        button("Add New Period")
            .style(button::primary)
            .on_press(Message::Dashboard(DashboardMessage::TeachingPeriod(TeachingPeriodMessage::ShowAddPeriodDialog))),
    ]
    .spacing(10)
    .align_y(Vertical::Center);

    let search_and_filters = row![
        text_input("Search periods...", &state.search_query)
            .on_input(|s| Message::Dashboard(DashboardMessage::TeachingPeriod(TeachingPeriodMessage::UpdateSearchQuery(s))))
            .width(Length::Fixed(300.0)),
        Space::with_width(Length::Fill),
    ]
    .spacing(10)
    .align_y(Vertical::Center);

    let periods_list: Element<'_, Message> = if state.teaching_periods.is_empty() {
        container(
            column![
                text("No teaching periods found").size(18),
                text("Add your first teaching period to get started").size(14)
                    .style(|theme| iced::widget::text::secondary(theme)),
            ]
            .spacing(10)
            .align_x(Alignment::Center)
        )
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    } else {
        let periods = state.teaching_periods
            .iter()
            .filter(|period| {
                if state.search_query.is_empty() {
                    true
                } else {
                    period.subject.to_lowercase().contains(&state.search_query.to_lowercase()) ||
                    period.class.to_lowercase().contains(&state.search_query.to_lowercase())
                }
            })
            .fold(column![], |col, period| {
                let period_card = container(
                    column![
                        row![
                            text(format!("{} - {}", period.subject, period.class)).size(16),
                            Space::with_width(Length::Fill),
                            text(format!("₵{:.2}", period.rate)).size(14)
                                .style(|theme| iced::widget::text::secondary(theme)),
                        ],
                        row![
                            text(format!("Date: {}", period.date.format("%d/%m/%Y"))).size(12),
                            Space::with_width(Length::Fixed(20.0)),
                            text(format!("Time: {}:00 - {}:00", period.start_time, period.end_time)).size(12),
                        ],
                        row![
                            text(format!("Teacher: {}", period.teacher_name)).size(12)
                                .style(|theme| iced::widget::text::secondary(theme)),
                            Space::with_width(Length::Fill),
                            button("Edit")
                                .style(button::text)
                                .on_press(Message::Dashboard(DashboardMessage::TeachingPeriod(TeachingPeriodMessage::EditPeriod(period.id.clone())))),
                            button("Delete")
                                .style(button::danger)
                                .on_press(Message::Dashboard(DashboardMessage::TeachingPeriod(TeachingPeriodMessage::DeletePeriod(period.id.clone())))),
                        ]
                        .spacing(10),
                    ]
                    .spacing(8)
                    .padding(15)
                )
                .style(container::bordered_box)
                .width(Length::Fill);

                col.push(period_card)
            })
            .spacing(10);

        scrollable(periods).height(Length::Fill).into()
    };

    let content = column![
        header,
        Space::with_height(Length::Fixed(20.0)),
        search_and_filters,
        Space::with_height(Length::Fixed(20.0)),
        periods_list,
    ]
    .spacing(10)
    .padding(20);

    if state.is_loading {
        container(
            text("Loading teaching periods...")
                .size(16)
        )
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    } else {
        content.into()
    }
}
