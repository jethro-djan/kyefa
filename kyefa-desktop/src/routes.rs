use reqwest::{Client, StatusCode};
use serde_json::json;
use serde::{Deserialize, Serialize};
use reqwest;
use std::path::PathBuf;
use reqwest::multipart::{Form, Part};
use rfd::FileDialog;
use umya_spreadsheet::*;
use umya_spreadsheet::writer;

use kyefa_models::{
    UserAccount, UserResponse, Gender, ClassLevel,
    Student, CreateStudentPayload, UpdateStudentPayload,
};
use crate::error::{LoginError, AppError, BackendError};

use once_cell::sync::Lazy;

static API_BASE_URL: Lazy<String> = Lazy::new(|| {
    dotenvy::dotenv().ok();
    std::env::var("KYEFA_BACKEND_URL")
        .expect("KYEFA_BACKEND_URL must be set in .env or environment variables")
});

pub async fn login(username: &str, password: &str) -> Result<UserResponse, LoginError> {
    let client = Client::new();
    let response = client
        .post("http://127.0.0.1:3050/login")
        .json(&json!({
            "username": username,
            "password": password,
        }))
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status() == StatusCode::OK {
                // let response_text = response.text().await.map_err(|e| 
                //     LoginError::ServerError(format!("Failed to read response: {}", e))
                // )?;
                
                // println!("Server response: {}", response_text); 
                
                // match serde_json::from_str::<UserResponse>(&response_text) {
                //     Ok(user) => Ok(user),
                //     Err(e) => {
                //         println!("JSON parse error: {}", e);
                //         Err(LoginError::ServerError(format!("Failed to parse server response: {}", e)))
                //     }
                // }

                match response.json::<UserResponse>().await {
                    Ok(user) => Ok(user),
                    Err(_) => Err(LoginError::ServerError("Failed to parse server response.".to_string())),
                }
            } else if response.status() == StatusCode::UNAUTHORIZED {
                Err(LoginError::InvalidCredentials("Incorrect username or password.".to_string()))
            } else if response.status() == StatusCode::NOT_FOUND {
                Err(LoginError::UserNotFound("User with that username does not exist.".to_string()))
            } else {
                Err(LoginError::ServerError(format!("Server returned an unexpected status: {}", response.status())))
            }
        }
        Err(e) => Err(LoginError::NetworkIssue(format!("Could not connect to the server: {}", e))),
    }
}

pub async fn create_student(payload: CreateStudentPayload) -> Result<Student, AppError> {
    let client = reqwest::Client::new();
    let res = client.post(&format!("{}/students", *API_BASE_URL)) // Use *API_BASE_URL
        .json(&payload)
        .send()
        .await
        .map_err(|e| AppError::NetworkIssue(e.to_string()))?; // Changed to NetworkIssue

    if res.status().is_success() {
        let student: Student = res.json()
            .await
            .map_err(|e| AppError::SerializationError(format!("Failed to parse student creation response: {}", e)))?;
        Ok(student)
    } else {
        let error_text = res.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        if let Ok(backend_error) = serde_json::from_str::<BackendError>(&error_text) { // Use BackendError
            Err(AppError::BackendError(backend_error.message))
        } else {
            Err(AppError::NetworkIssue(format!("Failed to create student: {}", error_text))) // Changed to NetworkIssue
        }
    }
}

pub async fn fetch_all_students() -> Result<Vec<Student>, AppError> {
    let client = reqwest::Client::new();
    let res = client.get(&format!("{}/students", *API_BASE_URL)) // Use *API_BASE_URL
        .send()
        .await
        .map_err(|e| AppError::NetworkIssue(e.to_string()))?; // Changed to NetworkIssue

    if res.status().is_success() {
        let students: Vec<Student> = res.json()
            .await
            .map_err(|e| AppError::SerializationError(format!("Failed to parse students list: {}", e)))?;
        Ok(students)
    } else {
        let error_text = res.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        if let Ok(backend_error) = serde_json::from_str::<BackendError>(&error_text) { // Use BackendError
            Err(AppError::BackendError(backend_error.message))
        } else {
            Err(AppError::NetworkIssue(format!("Failed to fetch students: {}", error_text))) // Changed to NetworkIssue
        }
    }
}

pub async fn update_student(payload: UpdateStudentPayload) -> Result<Student, AppError> {
    let client = reqwest::Client::new();
    let res = client
        .put(&format!("{}/students", *API_BASE_URL)) // Matches backend route
        .json(&payload)
        .send()
        .await
        .map_err(|e| AppError::NetworkIssue(e.to_string()))?;

    if res.status().is_success() {
        res.json::<Student>()
            .await
            .map_err(|e| AppError::SerializationError(e.to_string()))
    } else {
        let error_text = res.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        if let Ok(backend_error) = serde_json::from_str::<BackendError>(&error_text) {
            Err(AppError::BackendError(backend_error.message))
        } else {
            Err(AppError::NetworkIssue(error_text))
        }
    }
}


pub async fn import_students_from_excel(path: PathBuf) -> Result<(), AppError> {
    let client = reqwest::Client::new();

    let file_bytes = tokio::fs::read(&path)
        .await
        .map_err(|e| AppError::IoError(format!("Failed to read file: {}", e)))?;

    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("students.xlsx");

    let part = Part::bytes(file_bytes)
        .file_name(file_name.to_string())
        .mime_str("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
        .map_err(|e| AppError::IoError(format!("Invalid MIME: {}", e)))?;

    let form = Form::new().part("file", part);

    let res = client
        .post(&format!("{}/students/import", *API_BASE_URL))
        .multipart(form)
        .send()
        .await
        .map_err(|e| AppError::NetworkIssue(e.to_string()))?;

    if res.status().is_success() {
        Ok(())
    } else {
        let text = res.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        if let Ok(backend_error) = serde_json::from_str::<BackendError>(&text) {
            Err(AppError::BackendError(backend_error.message))
        } else {
            Err(AppError::NetworkIssue(format!("Upload failed: {}", text)))
        }
    }
}

pub async fn pick_and_upload_excel_file() -> Result<(), AppError> {
    let file = FileDialog::new()
        .add_filter("Excel Files", &["xlsx"])
        .pick_file();

    match file {
        Some(path) => import_students_from_excel(path).await,
        None => Err(AppError::IoError("No file selected".into())),
    }
}


pub fn generate_excel_template(path: &PathBuf) -> Result<(), AppError> {
    let mut book = new_file();

    // Define validation options upfront
    let gender_options = ["Male", "Female"];
    let class_options = [
        "LowerSecondaryYear8", "LowerSecondaryYear9", "IGCSE1", "IGCSE2",
        "WASSCE1", "WASSCE2", "WASSCE3", "ALevel1", "ALevel2",
    ];

    // Create and setup validation sheet
    let validation_sheet_name = "ValidationLists";

    book.new_sheet(validation_sheet_name)
        .map_err(|e| AppError::Configuration(format!("Failed to create sheet: {}", e)))?;

    {
        let validation_sheet = book
            .get_sheet_by_name_mut(validation_sheet_name)
            .ok_or_else(|| AppError::Configuration("Validation sheet not found".to_string()))?;

        validation_sheet.set_sheet_state(SheetStateValues::Hidden.get_value_string().to_string());

        for (i, &value) in gender_options.iter().enumerate() {
            let coord = format!("A{}", i + 1);
            validation_sheet.get_cell_mut(coord.as_str()).set_value(value);
        }

        for (i, &value) in class_options.iter().enumerate() {
            let coord = format!("B{}", i + 1);
            validation_sheet.get_cell_mut(coord.as_str()).set_value(value);
        }
    }

    // Setup main sheet with headers, data, and validations
    let sheet = book
        .get_sheet_by_name_mut("Sheet1")
        .ok_or_else(|| AppError::Configuration("Main sheet 'Sheet1' not found".to_string()))?;

    // Set headers
    let headers = [
        ("A1", "FirstName"),
        ("B1", "Surname"),
        ("C1", "OtherNames"),
        ("D1", "Gender"),
        ("E1", "ClassLevel"),
    ];

    for (cell, value) in headers {
        sheet.get_cell_mut(cell).set_value(value);
    }

    // Add sample data
    let sample_data = [
        ("A2", "John"),
        ("B2", "Doe"),
        ("C2", "Kwabena"),
        ("D2", "Male"),
        ("E2", "IGCSE1"),
    ];

    for (cell, value) in sample_data {
        sheet.get_cell_mut(cell).set_value(value);
    }

    // Create data validations
    let mut data_validations = DataValidations::default();

    // Helper function to create validation
    let create_validation = |formula: String, range: &str, title: &str, message: &str| {
        let mut validation = DataValidation::default();
        validation.set_type(DataValidationValues::List);
        validation.set_formula1(formula);
        validation.set_allow_blank(true);
        validation.set_show_error_message(true);
        validation.set_error_title(title);
        validation.set_error_message(message);
        validation.set_show_input_message(false);

        let mut sequence = SequenceOfReferences::default();
        sequence.set_sqref(range);
        validation.set_sequence_of_references(sequence);
        validation
    };

    // Add gender validation
    data_validations.add_data_validation_list(create_validation(
        format!("{}!$A$1:$A${}", validation_sheet_name, gender_options.len()),
        "D2:D1000",
        "Invalid Gender",
        "Please select either Male or Female from the dropdown.",
    ));

    // Add class validation
    data_validations.add_data_validation_list(create_validation(
        format!("{}!$B$1:$B${}", validation_sheet_name, class_options.len()),
        "E2:E1000",
        "Invalid Class Level",
        "Please select a valid class level from the dropdown.",
    ));

    sheet.set_data_validations(data_validations);

    writer::xlsx::write(&book, path)
        .map_err(|e| AppError::IoError(format!("Failed to write file: {}", e)))
}


// pub fn generate_excel_template(path: &PathBuf) -> Result<(), AppError> {
//     let mut book = new_file();
//     
//     // Set headers on Sheet1 without holding long-lived reference
//     {
//         let sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();
//         sheet.get_cell_mut("A1").set_value("FirstName");
//         sheet.get_cell_mut("B1").set_value("Surname");
//         sheet.get_cell_mut("C1").set_value("OtherNames");
//         sheet.get_cell_mut("D1").set_value("Gender");
//         sheet.get_cell_mut("E1").set_value("ClassLevel");
//     } // sheet reference drops here
// 
//     // Create validation sheet
//     let mut workbook_clone = book.clone();
//     let validation_sheet_index = workbook_clone.new_sheet("ValidationLists")
//         .map_err(|e| AppError::Configuration(format!("Failed to create sheet: {}", e)))?;
//     
//     // Get validation sheet
//     let validation_sheet = book.get_sheet_by_name_mut(&validation_sheet_index.get_sheet_id())
//         .ok_or_else(|| AppError::Configuration("Validation sheet not found".to_string()))?;
//     
//     // Set sheet state to hidden
//     validation_sheet.set_sheet_state(SheetStateValues::Hidden.get_value_string().to_string());
// 
//     // Define validation options
//     let gender_options = vec!["Male", "Female"];
//     let class_options = vec![
//         "LowerSecondaryYear8",
//         "LowerSecondaryYear9",
//         "IGCSE1",
//         "IGCSE2",
//         "WASSCE1",
//         "WASSCE2",
//         "WASSCE3",
//         "ALevel1",
//         "ALevel2",
//     ];
// 
//     // Write options to validation sheet
//     for (index, value) in gender_options.iter().enumerate() {
//         let coord = format!("A{}", index + 1);
//         validation_sheet.get_cell_mut(&*coord).set_value(*value);
//     }
//     for (index, value) in class_options.iter().enumerate() {
//         let coord = format!("B{}", index + 1);
//         validation_sheet.get_cell_mut(&*coord).set_value(*value);
//     }
// 
//     // Drop validation sheet reference before accessing main sheet again
//     let _ = validation_sheet;
// 
//     // Prepare data validations
//     let mut data_validations = DataValidations::default();
//     
//     // Gender validation
//     let mut gender_validation = DataValidation::default();
//     gender_validation.set_type(DataValidationValues::List);
//     gender_validation.set_formula1(format!("ValidationLists!$A$1:$A${}", gender_options.len()));
//     gender_validation.set_allow_blank(true);
//     gender_validation.set_show_error_message(true);
//     gender_validation.set_error_title("Invalid Gender");
//     gender_validation.set_error_message("Please select either Male or Female from the dropdown.");
//     gender_validation.set_show_input_message(false);
//     
//     let mut gender_sequence = SequenceOfReferences::default();
//     gender_sequence.set_sqref("D2:D1000");
//     gender_validation.set_sequence_of_references(gender_sequence);
//     data_validations.add_data_validation_list(gender_validation);
//     
//     // ClassLevel validation
//     let mut class_validation = DataValidation::default();
//     class_validation.set_type(DataValidationValues::List);
//     class_validation.set_formula1(format!("ValidationLists!$B$1:$B${}", class_options.len()));
//     class_validation.set_allow_blank(true);
//     class_validation.set_show_error_message(true);
//     class_validation.set_error_title("Invalid Class Level");
//     class_validation.set_error_message("Please select a valid class level from the dropdown.");
//     class_validation.set_show_input_message(false);
//     
//     let mut class_sequence = SequenceOfReferences::default();
//     class_sequence.set_sqref("E2:E1000");
//     class_validation.set_sequence_of_references(class_sequence);
//     data_validations.add_data_validation_list(class_validation);
//     
//     // Get main sheet again and set validations + data
//     let sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();
//     sheet.set_data_validations(data_validations);
//     
//     sheet.get_cell_mut("A2").set_value("John");
//     sheet.get_cell_mut("B2").set_value("Doe");
//     sheet.get_cell_mut("C2").set_value("Kwabena");
//     sheet.get_cell_mut("D2").set_value("Male");
//     sheet.get_cell_mut("E2").set_value("IGCSE1");
//     
//     writer::xlsx::write(&book, path).map_err(|e| AppError::IoError(format!("Failed to write file: {}", e)))
// }


pub fn pick_path_and_generate_excel_template() -> Result<(), AppError> {
    if let Some(path) = rfd::FileDialog::new()
        .set_file_name("student_import_template.xlsx")
        .add_filter("Excel", &["xlsx"])
        .save_file()
    {
        generate_excel_template(&path)
    } else {
        Err(AppError::IoError("No file path chosen.".into()))
    }
}

