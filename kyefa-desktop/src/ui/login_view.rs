use iced::{Element, Length, Theme, Background, Border, Color, border};
use iced::widget::{column, text, text_input, button, container, row, Space};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::Column;

use crate::{
    error::LoginError,
    app::{Message, LoginMessage, LoginState},
};

pub fn login_view(state: &LoginState) -> Element<'_, Message> {
    let username_field = text_input("Enter Username", &state.username_input)
        .width(400.0)
        .line_height(text::LineHeight::Relative(2.0))
        .size(15)
        .on_input(|s| Message::Login(LoginMessage::UsernameInputChanged(s)));

    let password_field = text_input("Enter Password", &state.password_input)
        .secure(true)
        .width(400.0)
        .line_height(text::LineHeight::Relative(2.0))
        .size(15)
        .on_input(|s| Message::Login(LoginMessage::PasswordInputChanged(s)));

    let login_button = button(text("Log in").size(15).center())
        .width(250.0)
        .padding(4)
        .style(move |theme, status| {
            if state.is_authenticating {
                button::primary(theme, button::Status::Disabled)
            } else {
                button::primary(theme, status)
            }
        })
        .on_press(Message::Login(LoginMessage::AttemptLogin));

    let login_fields = Column::new()
        .push(username_field)
        .push(password_field)
        .spacing(15);

    let mut fields_with_button = Column::new()
        .push(login_fields)
        .push(login_button)
        .align_x(Horizontal::Center)
        .spacing(20);

    if let Some(error) = &state.error {
        let error_message = match error {
            LoginError::UserNotFound(msg) => format!("User not found: {}", msg),
            LoginError::InvalidCredentials(msg) => format!("Invalid credentials: {}", msg),
            LoginError::NetworkIssue(msg) => format!("Network issue: {}", msg),
            LoginError::ServerError(msg) => format!("Server error: {}", msg),
        };
        let display_message = row![text(error_message).color(iced::Color::from_rgb(1.0, 0.0, 0.0))];
        fields_with_button = fields_with_button.push(display_message);
    }

    if state.is_authenticating {
        let authenticating_message = row![text("Authenticating...").size(18).color(iced::Color::from_rgb(0.0, 0.5, 1.0))];
        fields_with_button = fields_with_button.push(authenticating_message);
    }

    let login_info = Column::new()
        .push(text("Kyefa").size(30).center())
        .push(fields_with_button)
        .align_x(Horizontal::Center)
        .width(Length::Shrink)
        .spacing(20)
        .into();

    login_container(login_info).into()
}

fn login_container<'a>(content: Element<'a, Message>) -> Element<'a, Message> {
    use iced::widget::container;

    let form_container = container(content)
        .width(Length::Fixed(400.0))
        .padding(50)
        .style(login_box_style);

    container(form_container)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

fn login_box_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
        border: Border {
            color: Color::from_rgb(0.95, 0.95, 0.95),
            width: 1.0,
            radius: border::Radius::new(8.0),
        },
        ..container::Style::default()
    }
}
