use iced::{Element, Task, Theme, Settings};
use uuid::Uuid;

use crate::models::{UserAccount, UserProfile, UserRole, PersonName};
use crate::routes;
use crate::ui::{login_view, dashboard_view};
use crate::error::{AppError, LoginError, PasswordChangeError, DashboardError};

pub fn run() -> iced::Result {
    iced::application(KyefaApp::title, KyefaApp::update, KyefaApp::view)
        .antialiasing(true)
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
    LoginSuccess(UserAccount),
    LoginFailed(LoginError),

    Dashboard(DashboardMessage),

    Logout,
}

impl KyefaApp {
    fn new() -> (Self, Task<Message>) {
        (Self {
            state: AppState::Login(LoginState::new()),
            settings: AppSettings {},
            global_error: None,
        },
        Task::none())
    }

    fn title(&self) -> String {
        String::from("Kyefa")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Login(login_message) => {
                if let AppState::Login(login_state) = &mut self.state {
                    match login_message {
                        LoginMessage::UsernameInputChanged(username) => {
                            login_state.username_input = username;
                            Task::none()
                        }
                        LoginMessage::PasswordInputChanged(password) => {
                            login_state.password_input = password;
                            Task::none()
                        }
                        LoginMessage::AttemptLogin => {
                            // login_state.is_authenticating = true;
                            // let username = login_state.username_input.clone();
                            // let password = login_state.password_input.clone();
                            // Task::perform(
                            //     async move { routes::login(&username, &password).await },
                            //     |result| match result {
                            //         Ok(user) => Message::LoginSuccess(user),
                            //         Err(error) => Message::LoginFailed(error),
                            //     },
                            // )
                            let temp_user = UserAccount {
                                id: Uuid::new_v4(),
                                username: "admin".to_string(),
                                password_hash: "".to_string(),
                                role: UserRole::Admin,
                                is_active: true,
                                name: PersonName {
                                    first_name: "John".to_string(),
                                    surname: "Doe".to_string(),
                                    other_names: Some("Admin".to_string()),
                                },
                            };
                            self.state = AppState::Dashboard(DashboardState::new(temp_user));
                            Task::none()
                        }
                            
                    }
                } else {
                    Task::none()
                }
            }
            Message::LoginSuccess(user) => {
                self.state = AppState::Dashboard(DashboardState::new(user));
                Task::none()
            }
            Message::LoginFailed(error) => {
                if let AppState::Login(login_state) = &mut self.state {
                    login_state.error = Some(error);
                    login_state.is_authenticating = false;
                }
                Task::none()
            }
            Message::Dashboard(dashboard_message) => {
                if let AppState::Dashboard(dashboard_state) = &mut self.state {
                    dashboard_state.update(dashboard_message)
                } else {
                    Task::none()
                }
            }
            Message::Logout => {
                self.state = AppState::Login(LoginState::new());
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        match &self.state {
            AppState::Login(login_state) => login_view::login_view(login_state),
            AppState::Dashboard(dashboard_state) => dashboard_view::dashboard_view(dashboard_state),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LoginMessage {
    AttemptLogin,
    UsernameInputChanged(String),
    PasswordInputChanged(String),
}


#[derive(Debug)]
pub struct LoginState {
    pub username_input: String,
    pub password_input: String,
    pub error: Option<LoginError>,
    pub is_authenticating: bool,
}

impl LoginState {
    fn new() -> Self {
        Self {
            username_input: String::new(),
            password_input: String::new(),
            error: None,
            is_authenticating: false,
        }
    }
}

#[derive(Debug)]
pub struct DashboardState {
    pub current_view: DashboardView,
    pub student_manager: StudentManagerState,
    pub teaching_period_manager: TeachingPeriodManagerState,
    pub payment_tracking: PaymentTrackingState,
    pub user_access_manager: UserAccessManagerState,
    pub reports_analytics: ReportsAnalyticsState,
    pub active_user: UserProfile,
    pub error: Option<DashboardError>
}

impl DashboardState {
    fn new(user_account: UserAccount) -> Self {
        Self {
            current_view: DashboardView::Home,
            student_manager: StudentManagerState {},
            teaching_period_manager: TeachingPeriodManagerState {},
            payment_tracking: PaymentTrackingState {},
            user_access_manager: UserAccessManagerState {},
            reports_analytics: ReportsAnalyticsState {},
            active_user: user_account.into(),
            error: None,
        }
    }

    pub fn update(&mut self, message: DashboardMessage) -> Task<Message> {
        match message {
            DashboardMessage::NavigateToStudentManager => {
                self.current_view = DashboardView::StudentManager;
                Task::none()
            },
            DashboardMessage::NavigateToTeachingPeriodManager => {
                self.current_view = DashboardView::TeachingPeriodManager;
                Task::none()
            },
            DashboardMessage::NavigateToPaymentTracking => {
                self.current_view = DashboardView::PaymentTrackingManager;
                Task::none()
            },
            DashboardMessage::NavigateToReportsAnalytics => {
                self.current_view = DashboardView::ReportsAnalytics;
                Task::none()
            },
            DashboardMessage::NavigateToUserAccessManager => {
                self.current_view = DashboardView::UserAccessManager;
                Task::none()
            },
            DashboardMessage::NavigateToHome => {
                self.current_view = DashboardView::Home;
                Task::none()
            },
            DashboardMessage::ChangePassword => {
                // TODO: Implement ChangePassword logic
                Task::none()
            },
            DashboardMessage::PasswordChanged(_) => {
                // TODO: Implement PasswordChanged logic
                Task::none()
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {
        use iced::widget::text;
        text("Welcome to the Dashboard!").into()
    }
}

#[derive(Debug, Default)]
pub enum DashboardView {
    #[default]
    Home,
    StudentManager,
    TeachingPeriodManager,
    PaymentTrackingManager,
    UserAccessManager,
    ReportsAnalytics,
}

#[derive(Debug, Clone, Default)]
pub enum DashboardMessage {
    #[default]
    NavigateToHome,
    NavigateToStudentManager,
    NavigateToTeachingPeriodManager,
    NavigateToPaymentTracking,
    NavigateToReportsAnalytics,
    NavigateToUserAccessManager,

    ChangePassword,
    PasswordChanged(Result<(), PasswordChangeError>), 
}
  
#[derive(Debug)]
pub struct StudentManagerState {}

#[derive(Debug)]
pub struct TeachingPeriodManagerState {}

#[derive(Debug)]
pub struct PaymentTrackingState {}

#[derive(Debug)]
pub struct UserAccessManagerState {}

#[derive(Debug)]
pub struct ReportsAnalyticsState {}

#[derive(Debug)]
pub struct AppSettings {}
