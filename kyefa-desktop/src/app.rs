use iced::{Element, Task, Theme, Settings, Subscription};
use uuid::Uuid;
use std::str::FromStr;
use std::time::Duration;

use kyefa_models::{
    UserAccount, UserProfile, UserRole, 
    PersonName, UserResponse, Gender,
    ClassLevel, Student,
};
use crate::routes;
use crate::ui::{login_view, dashboard_view};
use crate::error::{AppError, LoginError, PasswordChangeError, DashboardError};
use crate::routes::CreateStudentPayload;

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
    LoginSuccess(UserResponse),
    LoginFailed(LoginError),

    Dashboard(DashboardMessage),

    Logout,

    AppLoading(bool),
    AppErrorOccurred(AppError),
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
        self.global_error = None;

        match message {
            Message::AppLoading(is_loading) => {
                match &mut self.state {
                    AppState::Login(state) => state.is_authenticating = is_loading,
                    AppState::Dashboard(state) => state.is_loading = is_loading,
                }
                Task::none()
            },
            Message::AppErrorOccurred(error) => {
                self.global_error = Some(error);
                Task::none()
            },
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
                            login_state.is_authenticating = true;
                            login_state.error = None;
                            let username = login_state.username_input.clone();
                            let password = login_state.password_input.clone();
                            Task::batch(vec![
                                Task::perform(
                                    async move { routes::login(&username, &password).await },
                                    |result| match result {
                                        Ok(user) => Message::LoginSuccess(user),
                                        Err(error) => Message::LoginFailed(error),
                                    },
                                ),
                                Task::perform(async { Message::AppLoading(true) }, |m| m),
                            ])
                            
                            // let temp_user = UserResponse {
                            //     id: Uuid::new_v4().to_string().to_string(),
                            //     username: "admin".to_string(),
                            //     role: UserRole::Admin,
                            //     is_active: true,
                            //     first_name: "John".to_string(),
                            //     surname: "Doe".to_string(),
                            //     other_names: Some("Admin".to_string()),
                            // };
                            // self.state = AppState::Dashboard(DashboardState::new(temp_user));
                            // Task::none()
                        }
                            
                    }
                } else {
                    Task::none()
                }
            }
            Message::LoginSuccess(user) => {
                self.state = AppState::Dashboard(DashboardState::new(user));
                Task::batch(vec![
                    Task::perform(async { Message::AppLoading(false) }, |m| m),
                    Task::perform(async {
                        Message::Dashboard(DashboardMessage::StudentManager(StudentManagerMessage::FetchStudents))
                    }, |m| m),
                ])
            }
            Message::LoginFailed(error) => {
                if let AppState::Login(login_state) = &mut self.state {
                    login_state.error = Some(error);
                    login_state.is_authenticating = false;
                }
                Task::perform(async { Message::AppLoading(false) }, |m| m)
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
    pub error: Option<DashboardError>,
    pub is_loading: bool,     
    pub global_app_error: Option<AppError>,
}

impl DashboardState {
    fn new(user_account: UserResponse) -> Self {
        Self {
            current_view: DashboardView::Home,
            student_manager: StudentManagerState::new(),
            teaching_period_manager: TeachingPeriodManagerState {},
            payment_tracking: PaymentTrackingState {},
            user_access_manager: UserAccessManagerState {},
            reports_analytics: ReportsAnalyticsState {},
            active_user: user_account.into(),
            error: None,
            is_loading: false,
            global_app_error: None,
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
            DashboardMessage::Loading(is_loading) => {
                self.is_loading = is_loading;
                Task::none()
            },
            DashboardMessage::ErrorOccurred(error) => {
                self.global_app_error = Some(error);
                Task::perform(async { Message::AppLoading(false) }, |m| m)
            },
            DashboardMessage::StudentManager(student_manager_msg) => {
                self.student_manager.update(student_manager_msg).map(|msg| Message::Dashboard(DashboardMessage::StudentManager(msg)))
            },
            DashboardMessage::StudentsFetched(students) => {
                self.is_loading = false;
                self.student_manager.students = students;
                Task::perform(async { Message::AppLoading(false) }, |m| m)
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

    Loading(bool),              
    ErrorOccurred(AppError),

    StudentManager(StudentManagerMessage),
    StudentsFetched(Vec<Student>),
}

#[derive(Debug, Clone)]
pub enum StudentManagerMessage {
    FirstNameInputChanged(String),
    SurnameInputChanged(String),
    OtherNamesInputChanged(String),
    GenderSelected(Gender),
    ClassLevelSelected(ClassLevel),
    SubmitNewStudent,      
    StudentCreated(Student),
    FetchStudents,         
    StudentFetchFailed(AppError),
    ClearForm,
    ClearSuccessMessage,
}
  
#[derive(Debug, Clone)]
pub struct StudentManagerState {
    pub first_name_input: String,
    pub surname_input: String,
    pub other_names_input: String,
    pub selected_gender: Option<Gender>,
    pub selected_class_level: Option<ClassLevel>,
    pub students: Vec<Student>,
    pub show_success_message: bool,
    pub form_error_message: Option<String>,
}

impl StudentManagerState {
    pub fn new() -> Self {
        Self {
            first_name_input: String::new(),
            surname_input: String::new(),
            other_names_input: String::new(),
            selected_gender: None,
            selected_class_level: None,
            students: Vec::new(),
            show_success_message: false,
            form_error_message: None,
        }
    }

    pub fn validate_form(&self) -> Result<(), String> {
        if self.first_name_input.trim().is_empty() {
            return Err("First name cannot be empty.".to_string());
        }
        if self.surname_input.trim().is_empty() {
            return Err("Surname cannot be empty.".to_string());
        }
        if self.selected_gender.is_none() {
            return Err("Please select a gender.".to_string());
        }
        if self.selected_class_level.is_none() {
            return Err("Please select a class level.".to_string());
        }
        Ok(())
    }

    pub fn clear_inputs(&mut self) {
        self.first_name_input = String::new();
        self.surname_input = String::new();
        self.other_names_input = String::new();
        self.selected_gender = None;
        self.selected_class_level = None;
        self.form_error_message = None;
        self.show_success_message = false;
    }

    // FIXED: Changed return type from Task<Message> to Task<StudentManagerMessage>
    pub fn update(&mut self, message: StudentManagerMessage) -> Task<StudentManagerMessage> {
        match message {
            StudentManagerMessage::FirstNameInputChanged(value) => {
                self.first_name_input = value;
                self.form_error_message = None;
                Task::none()
            },
            StudentManagerMessage::SurnameInputChanged(value) => {
                self.surname_input = value;
                self.form_error_message = None;
                Task::none()
            },
            StudentManagerMessage::OtherNamesInputChanged(value) => {
                self.other_names_input = value;
                self.form_error_message = None;
                Task::none()
            },
            StudentManagerMessage::GenderSelected(gender) => {
                self.selected_gender = Some(gender);
                self.form_error_message = None;
                Task::none()
            },
            StudentManagerMessage::ClassLevelSelected(class_level) => {
                self.selected_class_level = Some(class_level);
                self.form_error_message = None;
                Task::none()
            },
            StudentManagerMessage::SubmitNewStudent => {
                // Client-side validation
                match self.validate_form() {
                    Ok(_) => {
                        self.form_error_message = None;
                        self.show_success_message = false;
                        let payload = CreateStudentPayload {
                            first_name: self.first_name_input.clone(),
                            surname: self.surname_input.clone(),
                            other_names: if self.other_names_input.is_empty() {
                                None
                            } else {
                                Some(self.other_names_input.clone())
                            },
                            gender: self.selected_gender.clone().unwrap(),
                            class_level: self.selected_class_level.clone().unwrap(),
                        };

                        Task::perform(
                            async move { 
                                match routes::create_student(payload).await {
                                    Ok(student) => StudentManagerMessage::StudentCreated(student),
                                    Err(_) => StudentManagerMessage::ClearForm, // Handle error appropriately
                                }
                            },
                            |msg| msg,
                        )
                    },
                    Err(validation_error) => {
                        self.form_error_message = Some(validation_error);
                        Task::none()
                    },
                }
            },
            StudentManagerMessage::StudentCreated(student) => {
                self.students.push(student); 
                self.students.sort_by(|a, b| a.name.surname.cmp(&b.name.surname)); 
                self.clear_inputs();
                self.show_success_message = true;
                
                // FIXED: Using standard library sleep instead of tokio
                Task::perform(
                    async {
                        std::thread::sleep(Duration::from_secs(3));
                        StudentManagerMessage::ClearSuccessMessage
                    },
                    |msg| msg,
                )
            },
            StudentManagerMessage::FetchStudents => {
                Task::none()
            },
            StudentManagerMessage::StudentFetchFailed(_error) => {
                Task::none()
            },
            StudentManagerMessage::ClearForm => {
                self.clear_inputs();
                Task::none()
            },
            StudentManagerMessage::ClearSuccessMessage => {
                self.show_success_message = false;
                Task::none()
            },
        }
    }
}

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
