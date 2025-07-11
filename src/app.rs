use iced::{Element, Task};

pub fn run() -> iced::Result {
    iced::application(KyefaApp::title, KyefaApp::update, KyefaApp::view)
        .theme(|_| Theme::Light)
        .settings(Settings { ..Settings::default() })
        .run_with(move || KyefaApp::new())
}

pub struct KyefaApp {
    pub state: AppState,
    pub settings: AppSettings,
    pub global_error: Option<AppError>,
}

#[derive(Debug)]
pub enum AppState {
    Login(LoginState),
    Dashboard(DashboardState),
}

#[derive(Debug, Clone)]
pub enum Message { 
    Login(LoginMessage),
    LoginSuccess,
    LoginFailed(LoginError),

    Dashboard(DashboardMessage),

    Logout,
}

pub enum LoginMessage {
    AttemptLogin,
    UsernameInputChanged(String),
    PasswordInputChanged(String),
}


pub struct LoginState {
    pub username_input: String,
    pub password_input: String,
    pub error: Option<LoginError>,
    pub is_authenticating: bool,
}

pub struct DashboardState {
    pub current_view: DashboardView,
    pub student_manager: StudentManagerState,
    pub teaching_period_manager: TeachingPeriodManagerState,
    pub payout_manager: PayoutManagerState,
    pub user_access_manager: UserAccessManagerState,
    pub reports_analytics: ReportsAnalyticsState,
    pub active_user_role: UserRole,
}

pub enum AppError {
    Login(LoginError),
    Dashboard(DashboardError),
    Configuration(String),
}

pub enum LoginError {
    UserNotFound,
    InvalidCredentials,
    NetworkIssue,
    ServerError,
}

pub enum DashboardError {
    StudentNotFound,
    PaymentDataNotLoaded,
}
