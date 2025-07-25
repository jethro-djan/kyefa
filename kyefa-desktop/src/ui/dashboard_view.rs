use iced::{Element, Length, Color, Alignment, Padding};
use iced::widget::{
    column, row, text, button, container, 
    Space,
};
use iced::alignment::{Vertical};

use crate::app::{DashboardState, Message, DashboardMessage, DashboardView, StudentManagerMessage, TeachingPeriodMessage, PaymentTrackingMessage, ReportsAnalyticsMessage, UserAccessMessage};
use crate::ui;
use crate::ui::{
    home_view, 
    student_manager_view, 
    teaching_period_view, 
    payment_tracking_view, 
    reports_analytics_view, 
    user_access_view
};
use kyefa_models::UserRole;

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
        .filter(|(label, _, _)| {
            if *label == "User Access" {
                matches!(state.active_user.role, UserRole::Admin)
            } else {
                true
            }
        })
        .fold(column![], |col, (label, message, view)| {
            let is_active = std::mem::discriminant(&state.current_view) == std::mem::discriminant(&view);
            
            let icon_color = if is_active {
                Some(Color::WHITE)
            } else {
                None
            };
            
            let icon = match view {
                DashboardView::Home => ui::helper::home(20.0, 20.0, icon_color),
                DashboardView::StudentManager => ui::helper::student(20.0, 20.0, icon_color),
                DashboardView::TeachingPeriodManager => ui::helper::period(20.0, 20.0, icon_color),
                DashboardView::PaymentTrackingManager => ui::helper::payment(20.0, 20.0, icon_color),
                DashboardView::ReportsAnalytics => ui::helper::report(20.0, 20.0, icon_color),
                DashboardView::UserAccessManager => ui::helper::access(20.0, 20.0, icon_color),
            };
            
            let menu_item = button(
                row![
                    container(icon),
                    Space::with_width(Length::Fixed(20.0)),
                    text(label)
                ]
                .align_y(Vertical::Center)
            )
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
    .spacing(5);


    let user_menu = column![
        user_info,
        change_password_button,
        logout_button
    ]
    .spacing(10)
    .padding(10)
    .align_x(Alignment::Start);

    let app_brand = container(
        row![text("Kyefa").size(25),]
    )
    .padding(10)
    .height(50);

    let sidebar = container(
        column![app_brand, menu_items, Space::with_height(Length::Fill), user_menu]
            .width(280)
            .padding(Padding {
                top: 10.0,
                bottom: 50.0,
                left: 25.0,
                ..Padding::default()
            })
            .align_x(Alignment::Start)
    )
    .style(container::rounded_box)
    .center_y(Length::Fill);

    let main_content: Element<'_, Message> = match state.current_view {
        DashboardView::Home => home_view::home_view(state).into(),
        DashboardView::StudentManager => {
            student_manager_view::student_manager_view(&state.student_manager)
                .map(|msg| Message::Dashboard(msg))
        },
        DashboardView::TeachingPeriodManager => {
            teaching_period_view::teaching_period_view(&state.teaching_period_manager)
        },
        DashboardView::PaymentTrackingManager => {
            payment_tracking_view::payment_tracking_view(&state.payment_tracking)
        },
        DashboardView::ReportsAnalytics => {
            reports_analytics_view::reports_analytics_view(&state.reports_analytics)
        },
        DashboardView::UserAccessManager => {
            user_access_view::user_access_view(&state.user_access_manager)
        },
    };

    row![sidebar, main_content]
        .width(Length::Fill)
        .into()
}
