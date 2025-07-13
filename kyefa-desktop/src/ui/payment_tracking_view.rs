use iced::{Element, Length, Color, Alignment, Padding};
use iced::widget::{
    column, row, text, button, container, 
    text_input, scrollable, pick_list, Space
};
use iced::alignment::{Horizontal, Vertical};
use crate::app::{PaymentTrackingState, Message, DashboardMessage, PaymentTrackingMessage};
use kyefa_models::PaymentStatus;

pub fn payment_tracking_view(state: &PaymentTrackingState) -> Element<'_, Message> {
    let header = row![
        text("Payment Tracking").size(24),
        Space::with_width(Length::Fill),
        button("Record Payment")
            .style(button::primary)
            .on_press(Message::Dashboard(DashboardMessage::PaymentTracking(PaymentTrackingMessage::ShowPaymentDialog))),
    ]
    .spacing(10)
    .align_y(Vertical::Center);

    let search_and_filters = row![
        text_input("Search students...", &state.search_query)
            .on_input(|s| Message::Dashboard(DashboardMessage::PaymentTracking(PaymentTrackingMessage::UpdateSearchQuery(s))))
            .width(Length::Fixed(250.0)),
        Space::with_width(Length::Fixed(20.0)),
        pick_list(
            vec![
                PaymentStatus::Paid,
                PaymentStatus::Partial,
                PaymentStatus::NotPaid,
                PaymentStatus::Exempt,
            ],
            state.filter_status.clone(),
            |status| Message::Dashboard(DashboardMessage::PaymentTracking(PaymentTrackingMessage::FilterByStatus(Some(status))))
        )
        .placeholder("Filter by status..."),
        Space::with_width(Length::Fill),
    ]
    .spacing(10)
    .align_y(Vertical::Center);

    let payment_summary = row![
        summary_card("Total Expected", format!("₵{:.2}", 
            state.students.iter().map(|s| s.fee_amount).sum::<f64>())),
        summary_card("Total Received", format!("₵{:.2}", 
            state.payments.iter().map(|p| p.amount).sum::<f64>())),
        summary_card("Outstanding", format!("₵{:.2}", 
            state.students.iter().map(|s| s.fee_amount).sum::<f64>() - 
            state.payments.iter().map(|p| p.amount).sum::<f64>())),
        summary_card("Collection Rate", format!("{:.1}%", 
            if state.students.is_empty() { 0.0 } else {
                (state.payments.len() as f64 / state.students.len() as f64) * 100.0
            })),
    ]
    .spacing(15);

    let students_list: Element<'_, Message> = if state.students.is_empty() {
        container(
            column![
                text("No students found").size(18),
                text("Students will appear here once added").size(14)
                    .style(|theme| iced::widget::text::secondary(theme)),
            ]
            .spacing(10)
            .align_x(Alignment::Center)
        )
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    } else {
        let students = state.students
            .iter()
            .filter(|student| {
                let search_match = if state.search_query.is_empty() {
                    true
                } else {
                    format!("{} {}", student.name.first_name, student.name.surname)
                        .to_lowercase()
                        .contains(&state.search_query.to_lowercase())
                };

                let status_match = if let Some(filter_status) = &state.filter_status {
                    &student.payment_status == filter_status
                } else {
                    true
                };

                search_match && status_match
            })
            .fold(column![], |col, student| {
                let status_color = match student.payment_status {
                    PaymentStatus::Paid => Color::from_rgb(0.2, 0.8, 0.2),
                    PaymentStatus::Partial => Color::from_rgb(1.0, 0.6, 0.0),
                    PaymentStatus::NotPaid => Color::from_rgb(0.8, 0.2, 0.2),
                    PaymentStatus::Exempt => Color::from_rgb(0.5, 0.5, 0.5),
                };

                let student_card = container(
                    column![
                        row![
                            text(format!("{} {}", student.name.first_name, student.name.surname)).size(16),
                            Space::with_width(Length::Fill),
                            container(
                                text(format!("{:?}", student.payment_status)).size(12)
                                    .style(move |_| iced::widget::text::Style {
                                        color: Some(status_color),
                                    })
                            )
                            .padding(Padding::from([4, 8]))
                            .style(move |theme| container::Style {
                                border: iced::Border {
                                    color: status_color,
                                    width: 1.0,
                                    radius: 4.0.into(),
                                },
                                ..container::transparent(theme)
                            }),
                        ],
                        row![
                            text(format!("Class: {:?}", student.class_level)).size(12),
                            Space::with_width(Length::Fixed(20.0)),
                            text(format!("Fee: ₵{:.2}", student.fee_amount)).size(12),
                            Space::with_width(Length::Fill),
                            button("View Payments")
                                .style(button::text)
                                .on_press(Message::Dashboard(DashboardMessage::PaymentTracking(PaymentTrackingMessage::ViewStudentPayments(student.id.to_string())))),
                            button("Record Payment")
                                .style(button::primary)
                                .on_press(Message::Dashboard(DashboardMessage::PaymentTracking(PaymentTrackingMessage::RecordPayment(student.id.to_string())))),
                        ]
                        .spacing(10),
                    ]
                    .spacing(8)
                    .padding(15)
                )
                .style(container::bordered_box)
                .width(Length::Fill);

                col.push(student_card)
            })
            .spacing(10);

        scrollable(students).height(Length::Fill).into()
    };

    let content = column![
        header,
        Space::with_height(Length::Fixed(20.0)),
        payment_summary,
        Space::with_height(Length::Fixed(20.0)),
        search_and_filters,
        Space::with_height(Length::Fixed(20.0)),
        students_list,
    ]
    .spacing(10)
    .padding(20);

    if state.is_loading {
        container(
            text("Loading payment data...")
                .size(16)
        )
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    } else {
        content.into()
    }
}

fn summary_card(title: &str, value: String) -> Element<'_, Message> {
    container(
        column![
            text(title).size(12)
                .style(|theme| iced::widget::text::secondary(theme)),
            text(value).size(18),
        ]
        .spacing(5)
        .align_x(Alignment::Center)
    )
    .padding(15)
    .style(container::bordered_box)
    .width(Length::Fill)
    .into()
}
