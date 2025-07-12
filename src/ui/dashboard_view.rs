use iced::{Element, Length, Color, Border, Alignment, Padding};
use iced::widget::{
    column, row, text, button, container, 
    text_input, Space,
};
use iced::alignment::{Horizontal, Vertical};

use crate::app::{DashboardState, Message};
use crate::ui;

pub fn dashboard_view(state: &DashboardState) -> Element<'_, Message> {
    let logout_button = row![button("Logout")
        .on_press(Message::Logout)];

    let menu_items = column![
        text("Add Student"),
        text("Do something else")
    ]
    .spacing(20)
    .padding(10)
    .align_x(Alignment::Start);

    let sidebar = container(
        column![menu_items, Space::with_height(Length::Fill), logout_button]
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

    let main_area = container(text("Main area").size(20));

    let profile_icon = ui::helper::profile(25.0);
    let header = container(
        row![
            text("Kyefa").size(25), 
            Space::with_width(Length::Fill),
            profile_icon,
        ]
        .align_y(Vertical::Center)
        .width(Length::Fill)
    )
    .padding(10)
    .width(Length::Fill)
    // .style(container::rounded_box)
    .height(50);

    column![header, row![sidebar, main_area]]
        .width(Length::Fill)
        .into()
}
// pub fn dashboard_view(state: &DashboardState) -> Element<'_, Message> {
//     // Sidebar
//     let sidebar = column![
//         text("Sidebar Navigation").size(20),
//         // Add more navigation items here later
//     ]
//     .width(Length::Fixed(200.0))
//     .padding(20)
//     .spacing(10);
// 
//     // Payout System Content
//     let select_term_dropdown = row![
//         text("Select Term:").size(18),
//         text_input("Term 1 2025, Term 2 2024...", "").width(Length::Fixed(250.0))
//     ]
//     .spacing(10)
//     .align_y(Vertical::Center);
// 
//     let generate_payout_button = button(text("Generate Payout Preview").size(18))
//         .padding(10);
// 
//     // Disbursement Preview
//     let teacher_payouts_header = row![
//         text("Teacher").width(Length::Fill),
//         text("Base Share").width(Length::Fill),
//         text("Period Share").width(Length::Fill),
//         text("Total").width(Length::Fill),
//     ]
//     .spacing(10);
// 
//     let teacher_payout_row_1 = row![
//         text("Mr. Kobi").width(Length::Fill),
//         text("GHS 200").width(Length::Fill),
//         text("GHS 800").width(Length::Fill),
//         text("GHS 1000").width(Length::Fill),
//     ]
//     .spacing(10);
// 
//     let teacher_payout_row_2 = row![
//         text("Mrs. Amp").width(Length::Fill),
//         text("GHS 200").width(Length::Fill),
//         text("GHS 750").width(Length::Fill),
//         text("GHS 950").width(Length::Fill),
//     ]
//     .spacing(10);
// 
//     let teacher_payouts_table = column![
//         teacher_payouts_header,
//         teacher_payout_row_1,
//         teacher_payout_row_2,
//         text("...").width(Length::Fill),
//     ]
//     .spacing(5);
// 
//     let support_staff_header = row![
//         text("Role").width(Length::Fill),
//         text("Name").width(Length::Fill),
//         text("Fixed Share").width(Length::Fill),
//     ]
//     .spacing(10);
// 
//     let support_staff_row_1 = row![
//         text("Headteacher").width(Length::Fill),
//         text("Mr. Brown").width(Length::Fill),
//         text("GHS 500").width(Length::Fill),
//     ]
//     .spacing(10);
// 
//     let support_staff_row_2 = row![
//         text("Janitor").width(Length::Fill),
//         text("Ms. Eva").width(Length::Fill),
//         text("GHS 150").width(Length::Fill),
//     ]
//     .spacing(10);
// 
//     let support_staff_shares_table = column![
//         support_staff_header,
//         support_staff_row_1,
//         support_staff_row_2,
//     ]
//     .spacing(5);
// 
//     let confirm_disbursement_button = button(text("Confirm Disbursement").size(18))
//         .padding(10);
// 
//     let disbursement_preview = container(column![
//         text("Disbursement Preview").size(22).center(),
//         Space::with_height(Length::Fixed(15.0)),
//         text("Total Revenue (Term): GHS X,XXX.XX").size(18),
//         text("Total Funds to Disburse: GHS Y,YYY.YY").size(18),
//         Space::with_height(Length::Fixed(15.0)),
//         text("**Teacher Payouts:**").size(18),
//         teacher_payouts_table,
//         Space::with_height(Length::Fixed(15.0)),
//         text("**Support Staff Shares:**").size(18),
//         support_staff_shares_table,
//         Space::with_height(Length::Fixed(15.0)),
//         text("Remaining Balance: GHS Z.ZZ").size(18),
//         Space::with_height(Length::Fixed(20.0)),
//         confirm_disbursement_button,
//     ]
//     .spacing(10)
//     .padding(20))
//     .width(Length::Fill)
//     .height(Length::Shrink)
//     .style(container::Style {
//         border: Border {
//             width: 1.0,
//             color: Color::BLACK,
//             ..Border::default()
//         }
//         ..container::Style::default()
// 
//     });
// 
//     let disbursement_history_tab = button(text("Disbursement History Tab (Click to switch view)").size(18))
//         .padding(10);
// 
//     let payout_system_content = column![
//         text("Payout System").size(28).center(),
//         Space::with_height(Length::Fixed(20.0)),
//         select_term_dropdown,
//         Space::with_height(Length::Fixed(10.0)),
//         generate_payout_button,
//         Space::with_height(Length::Fixed(30.0)),
//         disbursement_preview,
//         Space::with_height(Length::Fixed(20.0)),
//         disbursement_history_tab,
//     ]
//     .width(Length::Fill)
//     .padding(20)
//     .spacing(15);
// 
//     // Main Row (Sidebar + Content)
//     row![sidebar, payout_system_content]
//         .width(Length::Fill)
//         .height(Length::Fill)
//         .into()
// }
