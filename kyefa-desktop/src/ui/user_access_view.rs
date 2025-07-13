use iced::{Element, Length, Color, Alignment, Padding};
use iced::widget::{
    column, row, text, button, container, 
    text_input, scrollable, pick_list, Space
};
use iced::alignment::{Horizontal, Vertical};
use crate::app::{UserAccessManagerState, Message, DashboardMessage, UserAccessMessage};
use kyefa_models::UserRole;

pub fn user_access_view(state: &UserAccessManagerState) -> Element<'_, Message> {
    let header = row![
        text("User Access Management").size(24),
        Space::with_width(Length::Fill),
        button("Add New User")
            .style(button::primary)
            .on_press(Message::Dashboard(DashboardMessage::UserAccess(UserAccessMessage::ShowAddUserDialog))),
    ]
    .spacing(10)
    .align_y(Vertical::Center);

    let search_bar = row![
        text_input("Search users...", &state.search_query)
            .on_input(|s| Message::Dashboard(DashboardMessage::UserAccess(UserAccessMessage::UpdateSearchQuery(s))))
            .width(Length::Fixed(300.0)),
        Space::with_width(Length::Fill),
    ]
    .spacing(10)
    .align_y(Vertical::Center);

    let users_list: Element<'_, Message> = if state.users.is_empty() {
        container(
            column![
                text("No users found").size(18),
                text("Add your first user to get started").size(14)
                    .style(|theme| iced::widget::text::secondary(theme)),
            ]
            .spacing(10)
            .align_x(Alignment::Center)
        )
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    } else {
        let users = state.users
            .iter()
            .filter(|user| {
                if state.search_query.is_empty() {
                    true
                } else {
                    format!("{} {} {}", user.name.first_name, user.name.surname, user.username)
                        .to_lowercase()
                        .contains(&state.search_query.to_lowercase())
                }
            })
            .fold(column![], |col, user| {
                let role_color = match user.role {
                    UserRole::Admin => Color::from_rgb(0.8, 0.2, 0.2),
                    UserRole::Staff => Color::from_rgb(0.2, 0.6, 0.9),
                    UserRole::Teacher => Color::from_rgb(0.2, 0.8, 0.2),
                    _ => Color::from_rgb(0.5, 0.5, 0.5),
                };

                let user_card = container(
                    column![
                        row![
                            text(format!("{} {}", user.name.first_name, user.name.surname)).size(16),
                            Space::with_width(Length::Fill),
                            container(
                                text(format!("{:?}", user.role)).size(12)
                                    .style(move |_| iced::widget::text::Style {
                                        color: Some(role_color),
                                    })
                            )
                            .padding(Padding::from([4, 8]))
                            .style(move |theme| container::Style {
                                border: iced::Border {
                                    color: role_color,
                                    width: 1.0,
                                    radius: 4.0.into(),
                                },
                                ..container::transparent(theme)
                            }),
                        ],
                        row![
                            text(format!("Username: {}", user.username)).size(12)
                                .style(|theme| iced::widget::text::secondary(theme)),
                            Space::with_width(Length::Fill),
                            button("Edit")
                                .style(button::text)
                                .on_press(Message::Dashboard(DashboardMessage::UserAccess(UserAccessMessage::EditUser(user.id.to_string())))),
                            button("Reset Password")
                                .style(button::secondary)
                                .on_press(Message::Dashboard(DashboardMessage::UserAccess(UserAccessMessage::ResetUserPassword(user.id.to_string())))),
                            button("Delete")
                                .style(button::danger)
                                .on_press(Message::Dashboard(DashboardMessage::UserAccess(UserAccessMessage::DeleteUser(user.id.to_string())))),
                        ]
                        .spacing(10),
                    ]
                    .spacing(8)
                    .padding(15)
                )
                .style(container::bordered_box)
                .width(Length::Fill);

                col.push(user_card)
            })
            .spacing(10);

        scrollable(users).height(Length::Fill).into()
    };

    let content = column![
        header,
        Space::with_height(Length::Fixed(20.0)),
        search_bar,
        Space::with_height(Length::Fixed(20.0)),
        users_list,
    ]
    .spacing(10)
    .padding(20);

    if state.is_loading {
        container(
            text("Loading users...")
                .size(16)
        )
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    } else {
        content.into()
    }
}
