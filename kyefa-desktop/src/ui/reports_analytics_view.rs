use iced::{Element, Length, Color, Alignment, Padding};
use iced::widget::{
    column, row, text, button, container, 
    text_input, scrollable, pick_list, Space
};
use iced::alignment::{Horizontal, Vertical};
use crate::app::{ReportsAnalyticsState, Message, DashboardMessage, ReportsAnalyticsMessage};
use kyefa_models::{ReportType, TeacherEarnings};

pub fn reports_analytics_view(state: &ReportsAnalyticsState) -> Element<'_, Message> {
    let header = row![
        text("Reports & Analytics").size(24),
        Space::with_width(Length::Fill),
        button("Export Report")
            .style(button::secondary)
            .on_press(Message::Dashboard(DashboardMessage::ReportsAnalytics(ReportsAnalyticsMessage::ExportReport))),
        button("Refresh Data")
            .style(button::primary)
            .on_press(Message::Dashboard(DashboardMessage::ReportsAnalytics(ReportsAnalyticsMessage::RefreshReports))),
    ]
    .spacing(10)
    .align_y(Vertical::Center);

    let report_tabs = row![
        tab_button("Projected Income", ReportType::ProjectedIncome, &state.selected_report_type),
        tab_button("Collection Status", ReportType::CollectionStatus, &state.selected_report_type),
        tab_button("Teacher Earnings", ReportType::TeacherEarnings, &state.selected_report_type),
        tab_button("Student Payments", ReportType::StudentPayments, &state.selected_report_type),
    ]
    .spacing(5);

    let filters = row![
        text_input("From Date (DD/MM/YYYY)", 
            state.date_filter_from.as_deref().unwrap_or(""))
            .on_input(|s| Message::Dashboard(DashboardMessage::ReportsAnalytics(ReportsAnalyticsMessage::UpdateDateFilterFrom(s))))
            .width(Length::Fixed(150.0)),
        text_input("To Date (DD/MM/YYYY)", 
            state.date_filter_to.as_deref().unwrap_or(""))
            .on_input(|s| Message::Dashboard(DashboardMessage::ReportsAnalytics(ReportsAnalyticsMessage::UpdateDateFilterTo(s))))
            .width(Length::Fixed(150.0)),
        Space::with_width(Length::Fill),
        button("Apply Filters")
            .style(button::secondary)
            .on_press(Message::Dashboard(DashboardMessage::ReportsAnalytics(ReportsAnalyticsMessage::ApplyReportFilters))),
    ]
    .spacing(10)
    .align_y(Vertical::Center);

    let overview_cards = row![
        metric_card("Total Students", state.total_students.to_string(), Color::from_rgb(0.2, 0.6, 0.9)),
        metric_card("Expected Revenue", format!("₵{:.2}", state.expected_revenue), Color::from_rgb(0.9, 0.6, 0.2)),
        metric_card("Actual Revenue", format!("₵{:.2}", state.total_revenue), Color::from_rgb(0.2, 0.8, 0.2)),
        metric_card("Collection Rate", format!("{:.1}%", state.collection_rate), Color::from_rgb(0.8, 0.2, 0.8)),
    ]
    .spacing(15);

    let report_content = match state.selected_report_type {
        ReportType::ProjectedIncome => projected_income_report(state),
        ReportType::CollectionStatus => collection_status_report(state),
        ReportType::TeacherEarnings => teacher_earnings_report(state),
        ReportType::StudentPayments => student_payments_report(state),
    };

    let content = column![
        header,
        Space::with_height(Length::Fixed(20.0)),
        overview_cards,
        Space::with_height(Length::Fixed(30.0)),
        report_tabs,
        Space::with_height(Length::Fixed(20.0)),
        filters,
        Space::with_height(Length::Fixed(20.0)),
        report_content,
    ]
    .spacing(10)
    .padding(20);

    if state.is_loading {
        container(
            text("Loading reports...")
                .size(16)
        )
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    } else {
        content.into()
    }
}

fn tab_button<'a>(label: &'a str, report_type: ReportType, current: &'a ReportType) -> Element<'a, Message> {
    let is_active = std::mem::discriminant(current) == std::mem::discriminant(&report_type);
    
    button(text(label))
        .style(move |theme, status| {
            if is_active {
                button::primary(theme, status)
            } else {
                button::secondary(theme, status)
            }
        })
        .on_press(Message::Dashboard(DashboardMessage::ReportsAnalytics(ReportsAnalyticsMessage::SelectReportType(report_type))))
        .into()
}

fn metric_card(title: &str, value: String, color: Color) -> Element<'_, Message> {
    container(
        column![
            text(title).size(12)
                .style(|theme| iced::widget::text::secondary(theme)),
            text(value).size(20)
                .style(move |_| iced::widget::text::Style {
                    color: Some(color),
                }),
        ]
        .spacing(8)
        .align_x(Alignment::Center)
    )
    .padding(20)
    .style(container::bordered_box)
    .width(Length::Fill)
    .into()
}

fn projected_income_report(state: &ReportsAnalyticsState) -> Element<'_, Message> {
    let total_projected = state.expected_revenue;
    let admin_share = total_projected * (30.0 / 100.0);
    let staff_share = total_projected * (20.0 / 100.0);
    let teacher_share = total_projected * (50.0 / 100.0);

    column![
        text("Projected Income Breakdown").size(18),
        Space::with_height(Length::Fixed(15.0)),
        income_row("Total Projected Income", format!("₵{:.2}", total_projected)),
        income_row("Admin Share (30%)", format!("₵{:.2}", admin_share)),
        income_row("Staff Share (20%)", format!("₵{:.2}", staff_share)),
        income_row("Teachers Share (50%)", format!("₵{:.2}", teacher_share)),
        Space::with_height(Length::Fixed(20.0)),
        text("Teacher Individual Earnings").size(16),
        Space::with_height(Length::Fixed(10.0)),
        teacher_earnings_list(&state.teacher_earnings),
    ]
    .spacing(5)
    .into()
}

fn collection_status_report(state: &ReportsAnalyticsState) -> Element<'_, Message> {
    let summary = &state.payment_summary;
    
    column![
        text("Collection Status Summary").size(18),
        Space::with_height(Length::Fixed(15.0)),
        collection_row("Total Expected", format!("₵{:.2}", summary.total_expected), Color::from_rgb(0.5, 0.5, 0.5)),
        collection_row("Total Received", format!("₵{:.2}", summary.total_received), Color::from_rgb(0.2, 0.8, 0.2)),
        collection_row("Total Pending", format!("₵{:.2}", summary.total_pending), Color::from_rgb(0.8, 0.2, 0.2)),
        Space::with_height(Length::Fixed(20.0)),
        text("Payment Status Breakdown").size(16),
        Space::with_height(Length::Fixed(10.0)),
        status_row("Paid", summary.paid_count, Color::from_rgb(0.2, 0.8, 0.2)),
        status_row("Partial", summary.partial_count, Color::from_rgb(1.0, 0.6, 0.0)),
        status_row("Not Paid", summary.unpaid_count, Color::from_rgb(0.8, 0.2, 0.2)),
        status_row("Exempt", summary.exempt_count, Color::from_rgb(0.5, 0.5, 0.5)),
    ]
    .spacing(5)
    .into()
}

fn teacher_earnings_report(state: &ReportsAnalyticsState) -> Element<'_, Message> {
    column![
        text("Teacher Earnings Report").size(18),
        Space::with_height(Length::Fixed(15.0)),
        teacher_earnings_list(&state.teacher_earnings),
    ]
    .spacing(5)
    .into()
}

fn student_payments_report(state: &ReportsAnalyticsState) -> Element<'_, Message> {
    column![
        text("Student Payments Report").size(18),
        Space::with_height(Length::Fixed(15.0)),
        text("Detailed payment history will be displayed here").size(14)
            .style(|theme| iced::widget::text::secondary(theme)),
    ]
    .spacing(5)
    .into()
}

fn income_row(label: &str, value: String) -> Element<'_, Message> {
    container(
        row![
            text(label).size(14),
            Space::with_width(Length::Fill),
            text(value).size(14),
        ]
        .align_y(Vertical::Center)
    )
    .padding(Padding::from([8, 0]))
    .style(container::bordered_box)
    .width(Length::Fill)
    .into()
}

fn collection_row(label: &str, value: String, color: Color) -> Element<'_, Message> {
    container(
        row![
            text(label).size(14),
            Space::with_width(Length::Fill),
            text(value).size(14)
                .style(move |_| iced::widget::text::Style {
                    color: Some(color),
                }),
        ]
        .align_y(Vertical::Center)
    )
    .padding(Padding::from([8, 0]))
    .style(container::bordered_box)
    .width(Length::Fill)
    .into()
}

fn status_row(label: &str, count: usize, color: Color) -> Element<'_, Message> {
    container(
        row![
            text(label).size(14),
            Space::with_width(Length::Fill),
            text(count.to_string()).size(14)
                .style(move |_| iced::widget::text::Style {
                    color: Some(color),
                }),
        ]
        .align_y(Vertical::Center)
    )
    .padding(Padding::from([8, 0]))
    .style(container::bordered_box)
    .width(Length::Fill)
    .into()
}

fn teacher_earnings_list(earnings: &[TeacherEarnings]) -> Element<'_, Message> {
    if earnings.is_empty() {
        container(
            text("No teacher earnings data available").size(14)
                .style(|theme| iced::widget::text::secondary(theme))
        )
        .padding(20)
        .center_x(Length::Fill)
        .into()
    } else {
        let earnings_list = earnings
            .iter()
            .fold(column![], |col, earning| {
                let earning_card = container(
                    row![
                        column![
                            text(&earning.teacher_name).size(14),
                            text(format!("{} periods", earning.total_periods)).size(12)
                                .style(|theme| iced::widget::text::secondary(theme)),
                        ]
                        .spacing(2),
                        Space::with_width(Length::Fill),
                        column![
                            text(format!("₵{:.2}", earning.total_earnings)).size(14),
                            text(format!("{:.1}% share", earning.share_percentage)).size(12)
                                .style(|theme| iced::widget::text::secondary(theme)),
                        ]
                        .spacing(2)
                        .align_x(Alignment::End),
                    ]
                    .align_y(Vertical::Center)
                )
                .padding(Padding::from([10, 15]))
                .style(container::bordered_box)
                .width(Length::Fill);

                col.push(earning_card)
            })
            .spacing(8);

        scrollable(earnings_list)
            .height(Length::Fixed(300.0))
            .into()
    }
}