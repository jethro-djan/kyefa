use iced::{Element, Task, Theme, Settings, Subscription};
use uuid::Uuid;
use std::str::FromStr;
use std::time::Duration;
use std::collections::HashMap;

use chrono::{DateTime, Utc, NaiveDate};
use kyefa_models::{
    User, UserAccount, UserProfile, UserRole, 
    PersonName, UserResponse, Gender,
    ClassLevel, Student, TeachingPeriod, Payment, 
    PaymentStatus, ReportType, RecentActivity, ActivityType, 
    PaymentSummary, TeacherEarnings, CreateStudentPayload,
    UpdateStudentPayload,
};
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
            settings: AppSettings::default(),
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
                            // login_state.is_authenticating = true;
                            // login_state.error = None;
                            // let username = login_state.username_input.clone();
                            // let password = login_state.password_input.clone();
                            // Task::batch(vec![
                            //     Task::perform(
                            //         async move { routes::login(&username, &password).await },
                            //         |result| match result {
                            //             Ok(user) => Message::LoginSuccess(user),
                            //             Err(error) => Message::LoginFailed(error),
                            //         },
                            //     ),
                            //     Task::perform(async { Message::AppLoading(true) }, |m| m),
                            // ])

                            let temp_user = UserResponse {
                                id: Uuid::new_v4().to_string().to_string(),
                                username: "admin".to_string(),
                                role: UserRole::Admin,
                                is_active: true,
                                first_name: "John".to_string(),
                                surname: "Doe".to_string(),
                                other_names: Some("Admin".to_string()),
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
            teaching_period_manager: TeachingPeriodManagerState::default(),
            payment_tracking: PaymentTrackingState::default(),
            user_access_manager: UserAccessManagerState::default(),
            reports_analytics: ReportsAnalyticsState::default(),
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
            // Placeholder for other message types
            _ => Task::none(),
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

#[derive(Debug, Clone)]
pub enum DashboardMessage {
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
    TeachingPeriod(TeachingPeriodMessage),
    PaymentTracking(PaymentTrackingMessage),
    UserAccess(UserAccessMessage),
    ReportsAnalytics(ReportsAnalyticsMessage),
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
    StudentCreationFailed(String),
    FetchStudents,         
    FetchedStudents(Vec<Student>),
    StudentFetchFailed(AppError),
    ClearForm,
    ClearSuccessMessage,

    EditStudent(Uuid),
    EditFirstNameInputChanged(String),
    EditSurnameInputChanged(String),
    EditOtherNamesInputChanged(String),
    EditGenderSelected(Gender),
    EditClassLevelSelected(ClassLevel),
    UpdateStudent(Uuid),
    DeleteStudent(Uuid),
    StudentUpdated(Result<Student, String>),
    StudentDeleted(Result<(), String>),
    CancelEdit,

    GenerateExcelTemplate,
    TemplateGenerationResult(Result<(), AppError>),
    ImportStudentsFromExcel,
    ImportResult(Result<(), AppError>),
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

    pub editing_student_id: Option<Uuid>,
    pub edit_first_name_input: String,
    pub edit_surname_input: String,
    pub edit_other_names_input: String,
    pub edit_selected_gender: Option<Gender>,
    pub edit_selected_class_level: Option<ClassLevel>,
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
            editing_student_id: None,
            edit_first_name_input: String::new(),
            edit_surname_input: String::new(),
            edit_other_names_input: String::new(),
            edit_selected_gender: None,
            edit_selected_class_level: None,
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

    pub fn validate_edit_form(&self) -> Result<(), String> {
        if self.edit_first_name_input.trim().is_empty() {
            return Err("First name cannot be empty.".to_string());
        }
        if self.edit_surname_input.trim().is_empty() {
            return Err("Surname cannot be empty.".to_string());
        }
        if self.edit_selected_gender.is_none() {
            return Err("Please select a gender.".to_string());
        }
        if self.edit_selected_class_level.is_none() {
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

    pub fn clear_edit_inputs(&mut self) {
        self.edit_first_name_input = String::new();
        self.edit_surname_input = String::new();
        self.edit_other_names_input = String::new();
        self.edit_selected_gender = None;
        self.edit_selected_class_level = None;
        self.editing_student_id = None;
        self.form_error_message = None;
    }

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
                                    Err(err) => StudentManagerMessage::StudentCreationFailed(err.to_string()),

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
                
                Task::perform(
                    async {
                        std::thread::sleep(Duration::from_secs(3));
                        StudentManagerMessage::ClearSuccessMessage
                    },
                    |msg| msg,
                )
            },
            StudentManagerMessage::StudentCreationFailed(err_msg) => {
                self.form_error_message = Some(format!("Failed to add student: {}", err_msg));
                self.show_success_message = false;
                Task::none()
            }

            StudentManagerMessage::FetchStudents => {
                Task::none()
            },
            StudentManagerMessage::FetchedStudents(_students) => {
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

            StudentManagerMessage::EditStudent(id) => {
                if let Some(student) = self.students.iter().find(|s| s.id == id) {
                    self.editing_student_id = Some(id);
                    self.edit_first_name_input = student.name.first_name.clone();
                    self.edit_surname_input = student.name.surname.clone();
                    self.edit_other_names_input = student.name.other_names.clone().unwrap_or_default();
                    self.edit_selected_gender = Some(student.gender.clone());
                    self.edit_selected_class_level = Some(student.class_level.clone());
                    self.form_error_message = None;
                }
                Task::none()
            },
            StudentManagerMessage::EditFirstNameInputChanged(value) => {
                self.edit_first_name_input = value;
                self.form_error_message = None;
                Task::none()
            },
            StudentManagerMessage::EditSurnameInputChanged(value) => {
                self.edit_surname_input = value;
                self.form_error_message = None;
                Task::none()
            },
            StudentManagerMessage::EditOtherNamesInputChanged(value) => {
                self.edit_other_names_input = value;
                self.form_error_message = None;
                Task::none()
            },
            StudentManagerMessage::EditGenderSelected(gender) => {
                self.edit_selected_gender = Some(gender);
                self.form_error_message = None;
                Task::none()
            },
            StudentManagerMessage::EditClassLevelSelected(class_level) => {
                self.edit_selected_class_level = Some(class_level);
                self.form_error_message = None;
                Task::none()
            },
            StudentManagerMessage::UpdateStudent(id) => {
                match self.validate_edit_form() {
                    Ok(_) => {
                        let payload = UpdateStudentPayload {
                            id,
                            first_name: self.edit_first_name_input.trim().to_string(),
                            surname: self.edit_surname_input.trim().to_string(),

                            other_names: self.edit_other_names_input.trim().is_empty()
                                .then(|| None)
                                .unwrap_or_else(|| Some(self.edit_other_names_input.trim().to_string())),

                            gender: self.edit_selected_gender.clone().unwrap(),
                            class_level: self.edit_selected_class_level.clone().unwrap(),
                        };

                        Task::perform(
                            async { routes::update_student(payload).await },
                            |result| match result {
                                Ok(student) => StudentManagerMessage::StudentUpdated(Ok(student)),
                                Err(e) => StudentManagerMessage::StudentUpdated(Err(e.to_string())),
                            },
                        )
                    },
                    Err(e) => {
                        self.form_error_message = Some(e);
                        Task::none()
                    },
                }
            },
            StudentManagerMessage::DeleteStudent(id) => {
                Task::perform(
                    async move {
                        let client = reqwest::Client::new();
                        match client
                            .delete(&format!("http://localhost:3000/api/students/{}", id))
                            .send()
                            .await
                        {
                            Ok(response) => {
                                if response.status().is_success() {
                                    Ok(())
                                } else {
                                    Err(response.text().await.unwrap_or_else(|_| "Unknown error".to_string()))
                                }
                            }
                            Err(e) => Err(e.to_string()),
                        }
                    },
                    |result| StudentManagerMessage::StudentDeleted(result),
                )
            },
            StudentManagerMessage::StudentUpdated(result) => {
                match result {
                    Ok(updated_student) => {
                        if let Some(index) = self.students.iter().position(|s| s.id == updated_student.id) {
                            self.students[index] = updated_student;
                            self.students.sort_by(|a, b| a.name.surname.cmp(&b.name.surname));
                        }
                        self.clear_edit_inputs();
                        self.show_success_message = true;
                        Task::perform(
                            async {
                                std::thread::sleep(Duration::from_secs(3));
                                StudentManagerMessage::ClearSuccessMessage
                            },
                            |msg| msg,
                        )
                    }
                    Err(e) => {
                        self.form_error_message = Some(e);
                        Task::none()
                    }
                }
            },
            StudentManagerMessage::StudentDeleted(result) => {
                match result {
                    Ok(_) => {
                        if let Some(id) = self.editing_student_id {
                            self.students.retain(|s| s.id != id);
                        }
                        self.clear_edit_inputs();
                        self.show_success_message = true;
                        Task::perform(
                            async {
                                std::thread::sleep(Duration::from_secs(3));
                                StudentManagerMessage::ClearSuccessMessage
                            },
                            |msg| msg,
                        )
                    }
                    Err(e) => {
                        self.form_error_message = Some(e);
                        Task::none()
                    }
                }
            },
            StudentManagerMessage::CancelEdit => {
                self.clear_edit_inputs();
                Task::none()
            }

            StudentManagerMessage::ImportStudentsFromExcel => {
                Task::perform(
                    routes::pick_and_upload_excel_file(),
                    |result| StudentManagerMessage::ImportResult(result),
                )
            }
            StudentManagerMessage::ImportResult(result) => {
                match result {
                    Ok(_) => {
                        self.show_success_message = true;
                        // optionally fetch all students again
                        Task::perform(routes::fetch_all_students(), |r| {
                            match r {
                                Ok(students) => StudentManagerMessage::FetchedStudents(students),
                                Err(e) => StudentManagerMessage::StudentFetchFailed(e),
                            }
                        })
                    }
                    Err(err) => {
                        self.form_error_message = Some(format!("Import failed: {}", err));
                        Task::none()
                    }
                }
            }

            StudentManagerMessage::GenerateExcelTemplate => {
                Task::perform(
                    async {
                        routes::pick_path_and_generate_excel_template()
                    },
                    |result| StudentManagerMessage::TemplateGenerationResult(result),
                )
            }
            StudentManagerMessage::TemplateGenerationResult(result) => {
                match result {
                    Ok(_) => {
                        self.show_success_message = true;
                    }
                    Err(err) => {
                        self.form_error_message = Some(format!("Template generation failed: {}", err));
                    }
                }
                Task::none()
            }

        }
    }
}

#[derive(Debug, Clone)]
pub enum ImportType {
    Students,
    TeachingPeriods,
}

#[derive(Debug, Clone)]
pub struct ImportPreview {
    pub headers: Vec<String>,
    pub sample_rows: Vec<Vec<String>>,
    pub total_rows: usize,
}

#[derive(Debug)]
pub struct TeachingPeriodManagerState {
    pub teaching_periods: Vec<TeachingPeriod>,
    pub teachers: Vec<User>,
    pub subjects: Vec<String>,
    pub classes: Vec<String>,
    pub new_period_teacher_id: Option<String>,
    pub new_period_subject: String,
    pub new_period_class: String,
    pub new_period_date: String,
    pub new_period_start_time: String,
    pub new_period_end_time: String,
    pub new_period_rate: String,
    pub search_query: String,
    pub selected_period_id: Option<String>,
    pub is_loading: bool,
    pub error_message: Option<String>,
    pub show_import_dialog: bool,
    pub import_file_path: Option<String>,
    pub import_preview: Option<ImportPreview>,
}

#[derive(Debug)]
pub struct PaymentTrackingState {
    pub students: Vec<Student>,
    pub payments: Vec<Payment>,
    pub selected_student_id: Option<String>,
    pub payment_amount: String,
    pub payment_method: String,
    pub payment_description: String,
    pub search_query: String,
    pub filter_status: Option<PaymentStatus>,
    pub filter_date_from: Option<String>,
    pub filter_date_to: Option<String>,
    pub is_loading: bool,
    pub error_message: Option<String>,
    pub show_payment_dialog: bool,
    pub selected_payment_id: Option<String>,
}

#[derive(Debug)]
pub struct UserAccessManagerState {
    pub users: Vec<User>,
    pub new_user_first_name: String,
    pub new_user_surname: String,
    pub new_user_username: String,
    pub new_user_password: String,
    pub new_user_role: kyefa_models::UserRole,
    pub selected_user_id: Option<String>,
    pub search_query: String,
    pub is_loading: bool,
    pub error_message: Option<String>,
    pub show_add_user_dialog: bool,
    pub show_edit_user_dialog: bool,
    pub show_delete_confirmation: bool,
}

#[derive(Debug)]
pub struct ReportsAnalyticsState {
    pub total_students: usize,
    pub total_revenue: f64,
    pub expected_revenue: f64,
    pub collection_rate: f64,
    pub recent_activities: Vec<RecentActivity>,
    pub payment_summary: PaymentSummary,
    pub teacher_earnings: Vec<TeacherEarnings>,
    pub selected_report_type: ReportType,
    pub date_filter_from: Option<String>,
    pub date_filter_to: Option<String>,
    pub selected_teacher_id: Option<String>,
    pub is_loading: bool,
    pub error_message: Option<String>,
}

#[derive(Debug)]
pub struct AppSettings {
    pub admin_share_percentage: f64,
    pub staff_share_percentage: f64,
    pub teacher_share_percentage: f64,
    pub default_payment_method: String,
    pub currency_symbol: String,
    pub date_format: String,
    pub backup_frequency: String,
    pub auto_backup_enabled: bool,
    pub theme: AppTheme,
}

#[derive(Debug, Clone)]
pub enum AppTheme {
    Light,
    Dark,
    System,
}

impl Default for TeachingPeriodManagerState {
    fn default() -> Self {
        Self {
            teaching_periods: Vec::new(),
            teachers: Vec::new(),
            subjects: vec![
                "Mathematics".to_string(),
                "English".to_string(),
                "Science".to_string(),
                "History".to_string(),
                "Geography".to_string(),
            ],
            classes: vec![
                "Grade 7".to_string(),
                "Grade 8".to_string(),
                "Grade 9".to_string(),
                "Grade 10".to_string(),
                "Grade 11".to_string(),
                "Grade 12".to_string(),
            ],
            new_period_teacher_id: None,
            new_period_subject: String::new(),
            new_period_class: String::new(),
            new_period_date: String::new(),
            new_period_start_time: String::new(),
            new_period_end_time: String::new(),
            new_period_rate: String::new(),
            search_query: String::new(),
            selected_period_id: None,
            is_loading: false,
            error_message: None,
            show_import_dialog: false,
            import_file_path: None,
            import_preview: None,
        }
    }
}

impl Default for PaymentTrackingState {
    fn default() -> Self {
        Self {
            students: Vec::new(),
            payments: Vec::new(),
            selected_student_id: None,
            payment_amount: String::new(),
            payment_method: "Cash".to_string(),
            payment_description: String::new(),
            search_query: String::new(),
            filter_status: None,
            filter_date_from: None,
            filter_date_to: None,
            is_loading: false,
            error_message: None,
            show_payment_dialog: false,
            selected_payment_id: None,
        }
    }
}

impl Default for UserAccessManagerState {
    fn default() -> Self {
        Self {
            users: Vec::new(),
            new_user_first_name: String::new(),
            new_user_surname: String::new(),
            new_user_username: String::new(),
            new_user_password: String::new(),
            new_user_role: kyefa_models::UserRole::Teacher,
            selected_user_id: None,
            search_query: String::new(),
            is_loading: false,
            error_message: None,
            show_add_user_dialog: false,
            show_edit_user_dialog: false,
            show_delete_confirmation: false,
        }
    }
}

impl Default for ReportsAnalyticsState {
    fn default() -> Self {
        Self {
            total_students: 0,
            total_revenue: 0.0,
            expected_revenue: 0.0,
            collection_rate: 0.0,
            recent_activities: Vec::new(),
            payment_summary: PaymentSummary {
                total_expected: 0.0,
                total_received: 0.0,
                total_pending: 0.0,
                paid_count: 0,
                partial_count: 0,
                unpaid_count: 0,
                exempt_count: 0,
            },
            teacher_earnings: Vec::new(),
            selected_report_type: ReportType::ProjectedIncome,
            date_filter_from: None,
            date_filter_to: None,
            selected_teacher_id: None,
            is_loading: false,
            error_message: None,
        }
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            admin_share_percentage: 30.0,
            staff_share_percentage: 20.0,
            teacher_share_percentage: 50.0,
            default_payment_method: "Cash".to_string(),
            currency_symbol: "₵".to_string(),
            date_format: "DD/MM/YYYY".to_string(),
            backup_frequency: "Daily".to_string(),
            auto_backup_enabled: true,
            theme: AppTheme::Light,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TeachingPeriodMessage {
    LoadPeriods,
    UpdateSearchQuery(String),
    ShowAddPeriodDialog,
    ShowImportDialog,
    AddPeriod,
    EditPeriod(String),
    DeletePeriod(String),
    UpdateTeacher(Option<String>),
    UpdateSubject(String),
    UpdateClass(String),
    UpdateDate(String),
    UpdateStartTime(String),
    UpdateEndTime(String),
    UpdateRate(String),
    ImportPeriods(String),
    PreviewImport(String),
    ConfirmImport,
    CancelImport,
}

#[derive(Debug, Clone)]
pub enum PaymentTrackingMessage {
    LoadPayments,
    UpdateSearchQuery(String),
    FilterByStatus(Option<PaymentStatus>),
    ShowPaymentDialog,
    RecordPayment(String),
    ViewStudentPayments(String),
    UpdatePaymentAmount(String),
    UpdatePaymentMethod(String),
    UpdatePaymentDescription(String),
    UpdateDateFilterFrom(String),
    UpdateDateFilterTo(String),
    SubmitPayment,
    CancelPayment,
    DeletePayment(String),
}

#[derive(Debug, Clone)]
pub enum UserAccessMessage {
    LoadUsers,
    UpdateSearchQuery(String),
    ShowAddUserDialog,
    ShowEditUserDialog(String),
    ShowDeleteConfirmation(String),
    AddUser,
    EditUser(String),
    DeleteUser(String),
    ResetUserPassword(String),
    UpdateNewUserFirstName(String),
    UpdateNewUserSurname(String),
    UpdateNewUserUsername(String),
    UpdateNewUserPassword(String),
    UpdateNewUserRole(UserRole),
    SubmitNewUser,
    SubmitEditUser,
    CancelUserDialog,
    ConfirmDeleteUser,
}

#[derive(Debug, Clone)]
pub enum ReportsAnalyticsMessage {
    LoadReports,
    RefreshReports,
    SelectReportType(ReportType),
    UpdateDateFilterFrom(String),
    UpdateDateFilterTo(String),
    UpdateTeacherFilter(Option<String>),
    ApplyReportFilters,
    ExportReport,
    GenerateProjectedIncome,
    GenerateCollectionStatus,
    GenerateTeacherEarnings,
    GenerateStudentPayments,
}
