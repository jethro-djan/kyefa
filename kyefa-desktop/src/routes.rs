use reqwest::{Client, StatusCode};
use serde_json::json;
use serde::{Deserialize, Serialize};
use reqwest;

use kyefa_models::{
    UserAccount, UserResponse, Gender, ClassLevel,
    Student, 
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

#[derive(Debug, Serialize)]
pub struct CreateStudentPayload {
    pub first_name: String,
    pub surname: String,
    pub other_names: Option<String>,
    pub gender: Gender,
    pub class_level: ClassLevel,
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
