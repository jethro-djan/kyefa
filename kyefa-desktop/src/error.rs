use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendError {
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum AppError {
    Login(LoginError),
    Dashboard(DashboardError),
    Configuration(String),
    NetworkIssue(String),  
    SerializationError(String),
    BackendError(String),  
    IoError(String),
}

#[derive(Debug, Clone)]
pub enum LoginError {
    UserNotFound(String),  
    InvalidCredentials(String),
    NetworkIssue(String), 
    ServerError(String),
}

#[derive(Debug, Clone)]
pub enum DashboardError {
    StudentNotFound,
    PaymentDataNotLoaded,
    PasswordChange(PasswordChangeError),
}

#[derive(Debug, Clone)]
pub enum PasswordChangeError {
    CurrentPasswordIncorrect,
    NewPasswordInvalid(String),
    NetworkError(String),
    ServerError(String),
    Unknown(String),
}

impl std::error::Error for AppError {}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Login(e) => write!(f, "Login Error: {}", e),
            AppError::Dashboard(e) => write!(f, "Dashboard Error: {:?}", e), // DashboardError needs Display impl or debug print
            AppError::Configuration(msg) => write!(f, "Configuration Error: {}", msg),
            AppError::NetworkIssue(msg) => write!(f, "Network Issue: {}", msg),
            AppError::SerializationError(msg) => write!(f, "Serialization Error: {}", msg),
            AppError::BackendError(msg) => write!(f, "Backend API Error: {}", msg),
            AppError::IoError(msg) => write!(f, "IO Error: {}", msg),
        }
    }
}

impl fmt::Display for LoginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoginError::UserNotFound(msg) => write!(f, "User not found: {}", msg),
            LoginError::InvalidCredentials(msg) => write!(f, "Invalid credentials: {}", msg),
            LoginError::NetworkIssue(msg) => write!(f, "Network problem: {}", msg),
            LoginError::ServerError(msg) => write!(f, "Server error: {}", msg),
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::NetworkIssue(err.to_string())
    }
}

impl From<reqwest::Error> for LoginError {
    fn from(err: reqwest::Error) -> Self {
        LoginError::NetworkIssue(err.to_string())
    }
}

impl From<BackendError> for AppError {
    fn from(err: BackendError) -> Self {
        AppError::BackendError(err.message)
    }
}

impl fmt::Display for DashboardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DashboardError::StudentNotFound => write!(f, "Student not found."),
            DashboardError::PaymentDataNotLoaded => write!(f, "Payment data could not be loaded."),
            DashboardError::PasswordChange(e) => write!(f, "Password change error: {}", e),
        }
    }
}

impl fmt::Display for PasswordChangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PasswordChangeError::CurrentPasswordIncorrect => write!(f, "Current password incorrect."),
            PasswordChangeError::NewPasswordInvalid(msg) => write!(f, "New password invalid: {}", msg),
            PasswordChangeError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            PasswordChangeError::ServerError(msg) => write!(f, "Server error: {}", msg),
            PasswordChangeError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl From<LoginError> for AppError {
    fn from(e: LoginError) -> Self {
        AppError::Login(e)
    }
}

impl From<DashboardError> for AppError {
    fn from(e: DashboardError) -> Self {
        AppError::Dashboard(e)
    }
}
