use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Login(LoginError),
    Dashboard(DashboardError),
    Configuration(String),
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
        write!(f, "{:?}", self)
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
