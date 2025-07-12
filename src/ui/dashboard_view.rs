use iced::{Element, Length, Color, Border, Alignment, Padding};
use iced::widget::{
    column, row, text, button, container, 
    text_input, Space,
};
use iced::alignment::{Horizontal, Vertical};

use crate::app::{DashboardState, Message, DashboardMessage, DashboardView};
use crate::ui;
use crate::ui::{
    home_view, 
    student_manager_view, 
    teaching_period_view, 
    payment_tracking_view, 
    reports_analytics_view, 
    user_access_view
};

pub fn dashboard_view(state: &DashboardState) -> Element<'_, Message> {
    let logout_button = row![
        button("Logout") 
            .style(move |theme, status| {
                button::danger(theme, button::Status::Active)
            })
            .on_press(Message::Logout)
    ];
    let change_password_button = row![
        button("Change Password") 
            .on_press(Message::Dashboard(DashboardMessage::ChangePassword))
    ];

    let menu_items_data = vec![
        ("Dashboard", DashboardMessage::NavigateToHome, DashboardView::Home),
        ("Student Management", DashboardMessage::NavigateToStudentManager, DashboardView::StudentManager),
        ("Teaching Periods", DashboardMessage::NavigateToTeachingPeriodManager, DashboardView::TeachingPeriodManager),
        ("Payment Tracking", DashboardMessage::NavigateToPaymentTracking, DashboardView::PaymentTrackingManager),
        ("Reports & Analytics", DashboardMessage::NavigateToReportsAnalytics, DashboardView::ReportsAnalytics),
        ("User Access", DashboardMessage::NavigateToUserAccessManager, DashboardView::UserAccessManager),
    ];

    let menu_items = menu_items_data
        .into_iter()
        .fold(column![], |col, (label, message, view)| {
            let is_active = std::mem::discriminant(&state.current_view) == std::mem::discriminant(&view);
            
            let menu_item = button(text(label))
                .style(move |theme, status| {
                    if is_active {
                        button::primary(theme, status)
                    } else {
                        button::text(theme, status)
                    }
                })
                .on_press(Message::Dashboard(message))
                .width(Length::Fill);
            
            col.push(menu_item)
        })
        .spacing(10)
        .padding(10)
        .align_x(Alignment::Start);

    let user_info = column![
        text(format!("{} {}", 
            state.active_user.name.first_name, 
            state.active_user.name.surname))
            .size(14),
        text(format!("Role: {:?}", state.active_user.role))
            .size(12)
            .style(|theme| iced::widget::text::secondary(theme)),
    ]
    .spacing(5)
    .align_x(Alignment::Start);


    let user_menu = column![
        user_info,
        change_password_button,
        logout_button
    ]
    .spacing(10)
    .padding(10);

    let app_brand = container(
        row![text("Kyefa").size(25),]
    )
    .padding(10)
    .height(50);

    let sidebar = container(
        column![app_brand, menu_items, Space::with_height(Length::Fill), user_menu]
            .width(200)
            .padding(Padding {
                top: 10.0,
                bottom: 50.0,
                ..Padding::default()
            })
            .align_x(Alignment::Center)
    )
    .style(container::rounded_box)
    .center_y(Length::Fill);

    let profile_icon = ui::helper::profile(25.0);

    let main_content = match state.current_view {
        DashboardView::Home => home_view::home_view(state),
        DashboardView::StudentManager => student_manager_view::student_manager_view(state),
        DashboardView::TeachingPeriodManager => teaching_period_view::teaching_period_view(state),
        DashboardView::PaymentTrackingManager => payment_tracking_view::payment_tracking_view(state),
        DashboardView::ReportsAnalytics => reports_analytics_view::reports_analytics_view(state),
        DashboardView::UserAccessManager => user_access_view::user_access_view(state),
        _ => text("Not implmented").into() 
    };

    row![sidebar, main_content]
        .width(Length::Fill)
        .into()
}
