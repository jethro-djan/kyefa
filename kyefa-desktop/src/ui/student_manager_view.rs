use iced::{Element, Length, alignment, Font, font::Weight, Color};
use iced::widget::{
    Column, Row, Text, TextInput, Button, PickList, Container, Scrollable, Space,
    column, row, text, text_input, button, pick_list, container, scrollable,
};
use crate::app::{DashboardMessage, StudentManagerMessage};
use crate::app::StudentManagerState;
use kyefa_models::{Gender, ClassLevel, Student};

pub fn student_manager_view<'a>(state: &'a StudentManagerState) -> Element<'a, DashboardMessage> {
    let header = row![
        text("Student Management").size(24),
        Space::with_width(Length::Fill),
        button("Download Excel Template") 
            .style(button::secondary)
            .on_press(DashboardMessage::StudentManager(StudentManagerMessage::GenerateExcelTemplate)),
        button("Import from Excel")
            .style(button::primary)
            .on_press(DashboardMessage::StudentManager(StudentManagerMessage::ImportStudentsFromExcel)),
    ];

    let add_student_section = column![
        header,
        text("Add New Student").size(20),
        text_input("First Name", &state.first_name_input)
            .on_input(|s| DashboardMessage::StudentManager(StudentManagerMessage::FirstNameInputChanged(s)))
            .padding(10),
        text_input("Surname", &state.surname_input)
            .on_input(|s| DashboardMessage::StudentManager(StudentManagerMessage::SurnameInputChanged(s)))
            .padding(10),
        text_input("Other Names (Optional)", &state.other_names_input)
            .on_input(|s| DashboardMessage::StudentManager(StudentManagerMessage::OtherNamesInputChanged(s)))
            .padding(10),
        row![
            text("Gender:"),
            pick_list(
                Gender::ALL.to_vec(),
                state.selected_gender.clone(),
                |gender| DashboardMessage::StudentManager(StudentManagerMessage::GenderSelected(gender)),
            )
            .width(Length::Fill),
            text("Class Level:"),
            pick_list(
                ClassLevel::ALL.to_vec(),
                state.selected_class_level.clone(),
                |class_level| DashboardMessage::StudentManager(StudentManagerMessage::ClassLevelSelected(class_level)),
            )
            .width(Length::Fill),
        ]
        .spacing(10),
        row![
            button(text("Add Student"))
                .on_press(DashboardMessage::StudentManager(StudentManagerMessage::SubmitNewStudent))
                .padding(10),
            button(text("Clear Form"))
                .on_press(DashboardMessage::StudentManager(StudentManagerMessage::ClearForm))
                .padding(10),
        ]
        .spacing(10),
    ]
    .spacing(10);

    // Display validation errors if any
    let mut form_feedback = column![].spacing(5);
    if let Some(error_msg) = &state.form_error_message {
        form_feedback = form_feedback.push(
            text(error_msg).style(|theme| {
                // Use a custom style function for error text
                iced::widget::text::Style {
                    color: Some(Color::from_rgb(1.0, 0.0, 0.0)),
                    ..iced::widget::text::default(theme)
                }
            })
        );
    }
    if state.show_success_message {
        form_feedback = form_feedback.push(
            text("Student Added Successfully!").style(|theme| {
                // Use a custom style function for success text
                iced::widget::text::Style {
                    color: Some(Color::from_rgb(0.0, 0.6, 0.0)),
                    ..iced::widget::text::default(theme)
                }
            })
        );
    }

    // Section for displaying existing students
    let students_list_header = row![
        text("Existing Students").size(24)
    ]
    .width(Length::Fill)
    .align_y(alignment::Vertical::Center); // Fixed: use align_y instead of align_items

    let students_table = column![
        row![
            text("Surname").width(Length::FillPortion(2)).font(Font {
                weight: Weight::Bold,
                ..Font::default()
            }),
            text("First Name").width(Length::FillPortion(2)).font(Font {
                weight: Weight::Bold,
                ..Font::default()
            }),
            text("Other Names").width(Length::FillPortion(2)).font(Font {
                weight: Weight::Bold,
                ..Font::default()
            }),
            text("Gender").width(Length::FillPortion(1)).font(Font {
                weight: Weight::Bold,
                ..Font::default()
            }),
            text("Class Level").width(Length::FillPortion(1)).font(Font {
                weight: Weight::Bold,
                ..Font::default()
            }),
        ]
        .spacing(10)
    ]
    .spacing(5);

    let mut students_rows = column![].spacing(2);
    if state.students.is_empty() {
        students_rows = students_rows.push(text("No students found."));
    } else {
        for student in &state.students {
            students_rows = students_rows.push(
                row![
                    text(&student.name.surname).width(Length::FillPortion(2)),
                    text(&student.name.first_name).width(Length::FillPortion(2)),
                    text(student.name.other_names.as_deref().unwrap_or("")).width(Length::FillPortion(2)),
                    text(student.gender.to_string()).width(Length::FillPortion(1)),
                    text(student.class_level.to_string()).width(Length::FillPortion(1)),
                ]
                .spacing(10)
            );
        }
    }

    let students_scrollable = scrollable(students_rows)
        .height(Length::Fill); // Allow the student list to scroll if it gets too long

    let content = column![
        add_student_section,
        form_feedback,
        Space::with_height(20),
        students_list_header,
        students_table,
        students_scrollable,
    ]
    .spacing(20)
    .padding(20);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
